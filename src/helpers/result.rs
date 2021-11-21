use std::fmt::Display;

pub trait ResultHelpers<E> {
	fn or_print(&self, action: &str);
	fn void(self) -> Result<(), E>;
}

impl<T, E: Display> ResultHelpers<E> for Result<T, E> {
	fn or_print(&self, action: &str) {
		if let Err(error) = self {
			eprintln!("Error trying to {}: {}", action, error);
		}
	}

	fn void(self) -> Result<(), E> {
		self.and(Ok(()))
	}
}
