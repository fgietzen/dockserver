use std::fmt::Debug;

pub(crate) fn both_or_none<S, T, E1: Debug, E2: Debug>(a: Result<S, E1>, b: Result<T, E2>) -> Option<(S, T)> {
	if a.is_err() || b.is_err() {
		return None;
	}
	return Some((a.unwrap(), b.unwrap()));
}
