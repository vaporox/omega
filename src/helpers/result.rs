use std::fmt::Display;

pub trait ResultHelpers {
	fn or_print(&self, action: &str);
}

impl<T, E: Display> ResultHelpers for Result<T, E> {
	fn or_print(&self, action: &str) {
		if let Err(error) = self {
			eprintln!("Error trying to {}: {}", action, error);
		}
	}
}
