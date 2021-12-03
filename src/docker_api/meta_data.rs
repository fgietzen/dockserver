use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct ImageMetaData {
	id: String,
	name: String,
	tag: String
}

impl Display for ImageMetaData {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		return write!(f, "|{}| {}:{}", self.id, self.name, self.tag);
	}
}

impl ImageMetaData {
	pub fn new<S: Into<String>, T: Into<String>, U: Into<String>>(id: S, name: T, tag: U) -> Self {
		return ImageMetaData {
			id: id.into(),
			name: name.into(),
			tag: tag.into()
		}
	}

	pub fn id(&self) -> &str {
		return &self.id;
	}

	pub fn name(&self) -> &str {
		return &self.name;
	}

	pub fn tag(&self) -> &str {
		return &self.tag;
	}
}

#[cfg(test)]
mod tests {

	use super::ImageMetaData;

	#[test]
	fn image_pull_display() {
		let p = ImageMetaData::new("a", "b", "c");
		assert_eq!(p.to_string(), "|a| b:c");
	}
}
