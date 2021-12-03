use crate::docker_api::ImageMetaData;
use crate::notifications::Notifier;

pub struct StdoutNotifier {
}

impl StdoutNotifier {
	pub fn new() -> Self {
		return StdoutNotifier {};
	}

	fn image_to_string(image: &ImageMetaData) -> String {
		return format!("Update available for {}:{}", image.name(), image.tag());
	}
}

impl Notifier for StdoutNotifier {
	fn notify(&mut self, image: &ImageMetaData) {
		println!("{}", StdoutNotifier::image_to_string(image));
	}
}
