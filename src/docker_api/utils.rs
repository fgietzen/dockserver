pub fn split_name_and_repo(name_tag: &str) -> Result<(&str, &str), &str> {
	let split = name_tag.split(":")
        .collect::<Vec<&str>>();

    if split.len() != 2 {
        return Err("Failed to split string into string and tag!");
    }

    return Ok((split[0], split[1]));
}

#[cfg(test)]
mod tests {
	use crate::docker_api::utils::split_name_and_repo;

	#[test]
	fn split_name_and_repo_simple() {
        let res = split_name_and_repo("abc:def");
        assert!(res.is_ok());
		assert_eq!(res.unwrap(), ("abc", "def"));
	}
}
