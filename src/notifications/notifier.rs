use crate::docker_api::ImageMetaData;

pub trait Notifier {
	fn notify(&mut self, image: &ImageMetaData);
}