use job_scheduler::{Job, Schedule};

use crate::docker_api;
use crate::docker_api::{ImageMetaData, ImagePull};
use crate::notifications::Notifier;

pub fn create_update_job<'a>(
	tokio_runtime: &'a tokio::runtime::Runtime,
	repository: &'a docker_api::DockerApi,
	mut notifier: Vec<Box<dyn Notifier>>,
	schedule: Schedule
) -> Job<'a> {
	return Job::new(schedule, move || {
		search_for_updates(tokio_runtime, repository, &mut notifier);
	});
}

fn search_for_updates(
	rt: &tokio::runtime::Runtime,
	repository: &docker_api::DockerApi,
	notifier: &mut Vec<Box<dyn Notifier>>
) {
	println!("Searching for image updates!");

	let images = rt.block_on(repository.list_images());
	let updated_images = images.iter()
		.map(|meta| rt.block_on(repository.update_image(&meta)));

	let images_with_update: Vec<ImageMetaData> = updated_images.filter_map(|pull|
		return if let ImagePull::UpdateAvailable(i) = pull {
			Some(i)
		} else {
			None
		}
	).collect();

	for n in notifier.iter_mut() {
		n.notify(&images_with_update);
	}
}
