use std::ops::Sub;
use std::str::FromStr;
use chrono::Local;
use cron::Schedule;
use shiplift::Docker;

mod utils;
mod docker_api;
mod notifications;
mod update_job;

fn handle_signal() {
	println!("Received SIGINT... Stopping");

	std::process::exit(0);
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

	let tokio_runtime = tokio::runtime::Runtime::new().unwrap();

	let schedule = std::env::var("SCHEDULE")
		.unwrap_or("0 0 5 * * * *".to_string());
	let schedule = Schedule::from_str(&schedule)
		.expect(&format!("Invalid schedule: {}", schedule));

	let docker = Docker::new();
	let repository = docker_api::DockerApi::new(docker);

	let mut notifier = create_notifiers();

	for next_update in schedule.upcoming(Local) {
		let seconds_to_next_update = next_update.sub(Local::now()).to_std().unwrap();
		std::thread::sleep(seconds_to_next_update);

		println!("Searching for image updates!");
		tokio_runtime.block_on(update_job::search_for_updates(&repository, &mut notifier));
	}

	println!("Exiting.")
}
