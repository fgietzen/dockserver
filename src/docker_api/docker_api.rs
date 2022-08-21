use std::fmt::{Display, Formatter};
use tokio_stream::StreamExt;
use shiplift::{PullOptions, ImageListOptions};

use super::ImageMetaData;
use super::utils;

#[derive(Debug)]
pub enum ImagePull {
	UpToDate(ImageMetaData),
	UpdateAvailable(ImageMetaData),
	Error(String),
}

impl Display for ImagePull {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let (status, info) = match self {
			ImagePull::UpToDate(m) => ("o", m.to_string()),
			ImagePull::UpdateAvailable(m) => ("u", m.to_string()),
			ImagePull::Error(e) => ("e", e.clone()),
		};
		return write!(f, "[{}] {}", status, info);
	}
}

pub struct DockerApi {
	docker_api: shiplift::Docker
}

impl DockerApi {
	pub fn new(docker_api: shiplift::Docker) -> Self {
		return DockerApi {
			docker_api
		};
	}

	pub async fn list_images(&self) -> Vec<ImageMetaData> {
		return self.docker_api.images().list(&ImageListOptions::default()).await
			.expect("Could not retrieve image list")
			.iter()
			.filter(|image| image.repo_tags.is_some())
			.flat_map(|image| image.repo_tags.as_ref().unwrap().iter()
				.map(|name_tag| utils::split_name_and_repo(name_tag))
				.map(|(name, tag)| ImageMetaData::new(&image.id, name, tag))
			).collect::<Vec<ImageMetaData>>();
	}

	pub async fn update_image(&self, image: &ImageMetaData) -> ImagePull {
		let old_id = image.id();

		let opts = PullOptions::builder()
			.image(image.name())
			.tag(image.tag())
			.build();
		let mut pull_status = self.docker_api.images().pull(&opts);

		while let Some(s) = pull_status.next().await {
			if let Err(e) = s {
				return ImagePull::Error(format!("{}: {}", image.name(), e.to_string()))
			}
		}

		let pulled_image = self.docker_api.images()
			.get(format!("{}:{}", image.name(), image.tag()))
			.inspect().await.unwrap();

		if old_id == &pulled_image.id {
			return ImagePull::UpToDate(image.clone());
		}

		let pulled_image_metadata =
			ImageMetaData::new(pulled_image.id, image.name().clone(), image.tag().clone());
		return ImagePull::UpdateAvailable(pulled_image_metadata);
	}
}

#[cfg(test)]
mod tests {
	use super::{ImagePull, ImageMetaData};

	#[test]
	fn image_pull_display_up_to_date() {
		let p = ImagePull::UpToDate(ImageMetaData::new("a", "b", "c"));
		assert_eq!(p.to_string(), "[o] |a| b:c");
	}

	#[test]
	fn image_pull_display_update_available() {
		let p = ImagePull::UpdateAvailable(ImageMetaData::new("a", "b", "c"));
		assert_eq!(p.to_string(), "[u] |a| b:c");
	}

	#[test]
	fn image_pull_display_error() {
		let p = ImagePull::Error("error 123".to_string());
		assert_eq!(p.to_string(), "[e] error 123");
	}
}
