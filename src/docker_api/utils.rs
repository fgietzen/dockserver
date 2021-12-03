pub fn split_name_and_repo<S: Into<String>>(name_tag: S) -> (String, String) {
	let name_tag = name_tag.into();
	let mut split = name_tag.split(":");

	let name = split.next().unwrap();
	let tag = split.next().unwrap();
	return (name.to_string(), tag.to_string());
}

#[cfg(test)]
mod tests {
	use crate::docker_api::utils::split_name_and_repo;

	#[test]
	fn split_name_and_repo_simple() {
		assert_eq!(split_name_and_repo("abc:def"), ("abc".to_string(), "def".to_string()));
	}
}
