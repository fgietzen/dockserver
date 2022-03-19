use crate::docker_api;
use crate::docker_api::ImagePull;
use crate::notifications::Notifier;

pub async fn search_for_updates(
	repository: &docker_api::DockerApi,
	notifier: &mut Vec<Box<dyn Notifier>>
) {
	let images = repository.list_images().await;

	let mut images_with_update = Vec::new();
	for meta in images {
		let pull = repository.update_image(&meta).await;

		if let ImagePull::UpdateAvailable(new_meta) = pull {
			images_with_update.push(new_meta);
		}
	}

	for n in notifier.iter_mut() {
		n.notify(&images_with_update);
	}
}
