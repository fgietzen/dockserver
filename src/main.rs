use std::time::Duration;
use std::sync::atomic::{Ordering, AtomicBool};

use job_scheduler::JobScheduler;
use shiplift::Docker;

mod utils;
mod docker_api;
mod notifications;
mod update_job;

static RUNNING: AtomicBool = AtomicBool::new(true);

fn is_running() -> bool {
	return RUNNING.load(Ordering::SeqCst);
}

fn handle_signal() {
	println!("Received SIGINT... Stopping");
	RUNNING.store(false, Ordering::SeqCst);
}

fn create_notifiers() -> Vec<Box<dyn notifications::Notifier>> {
	let mut notifier: Vec<Box<dyn notifications::Notifier>> = Vec::new();

	let stdout_notifier = notifications::StdoutNotifier::new();
	notifier.push(Box::new(stdout_notifier));
	println!("Added stdout notifier");

	#[cfg(feature = "telegram_notifier")] {
		let telegram_params = utils::both_or_none(
			std::env::var("TELEGRAM_BOT_ID"),
			std::env::var("TELEGRAM_CLIENT_ID")
		);
		if let Some((bot_id, client_id)) = telegram_params {
			let telegram_notifier = notifications::TelegramNotifier::new(bot_id, client_id);
			notifier.push(Box::new(telegram_notifier));
			println!("Added telegram notifier");
		}
	}

	return notifier;
}

fn main() {
	ctrlc::set_handler(handle_signal).expect("Error setting Ctrl-C handler");

	let schedule = std::env::var("UPDATE_SCHEDULE")
		.unwrap_or("0 0 4 * * * *".to_string()).parse::<job_scheduler::Schedule>()
		.expect("Could not parse update schedule!");

	let docker = Docker::new();
	let repository = docker_api::DockerApi::new(docker);

	let notifier = create_notifiers();

	let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
	let update_job = update_job::create_update_job(&tokio_runtime, &repository, notifier, schedule);

	let mut job_scheduler = JobScheduler::new();
	job_scheduler.add(update_job);
	println!("Registered update job");

	while is_running() {
		job_scheduler.tick();

		std::thread::sleep(Duration::from_millis(500));
	}

	println!("Exiting.")
}
