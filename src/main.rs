use serde::Deserialize;

const VERSION: &str = match option_env!("BUILD_VERSION") {
	Some(version) => version,
	None => env!("CARGO_PKG_VERSION"),
};

#[derive(Debug, Deserialize)]
struct Greeting<'a> {
	name: &'a str,
}

fn greet(json: &str) -> serde_json::Result<String> {
	let greeting: Greeting<'_> = serde_json::from_str(json)?;

	Ok(format!("hello, {}", greeting.name))
}

fn shout(message: &str) -> String {
	message.to_uppercase()
}

fn main() {
	match std::env::args().nth(1).as_deref() {
		Some("--version") => {
			println!("{VERSION}");

			return;
		},
		Some("--shout") => {
			match greet(r#"{"name":"world"}"#) {
				Ok(message) => println!("{}", shout(&message)),
				Err(error) => eprintln!("error: {error}"),
			}

			return;
		},
		Some("-h") => {
			println!("Usage: release-test [-h] [--version] [name]");

			return;
		},
		Some(name) => {
			println!("hello, {name}");

			return;
		},
		None => {},
	}

	match greet(r#"{"name":"world"}"#) {
		Ok(message) => println!("{message}"),
		Err(error) => {
			eprintln!("error: {error}");
			std::process::exit(1);
		},
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn greets_by_name() {
		assert_eq!(greet(r#"{"name":"test"}"#).unwrap(), "hello, test");
	}

	#[test]
	fn shouts_in_uppercase() {
		assert_eq!(shout("hello, test"), "HELLO, TEST");
	}

	#[test]
	fn rejects_invalid_json() {
		assert!(greet("{").is_err());
	}
}
