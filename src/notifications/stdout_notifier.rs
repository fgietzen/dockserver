use crate::docker_api::ImageMetaData;
use crate::notifications::Notifier;

pub struct StdoutNotifier {}

impl StdoutNotifier {
	pub fn new() -> Self {
		return StdoutNotifier {};
	}

	fn image_to_string(image: &ImageMetaData) -> String {
		return format!("Update available for {}:{}", image.name(), image.tag());
	}
}

impl Notifier for StdoutNotifier {
	fn notify(&mut self, images: &Vec<ImageMetaData>) {
		if images.is_empty() {
			println!("No updates available!");
			return;
		}

		for image in images {
			println!("{}", StdoutNotifier::image_to_string(image));
		}
	}
}
