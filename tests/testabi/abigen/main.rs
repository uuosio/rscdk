use std::{fs, path::Path};

extern crate contract;


extern "Rust" {
	pub fn __eosio_generate_abi() -> String;
}

fn main() -> Result<(), std::io::Error> {
	let abi = unsafe {
		__eosio_generate_abi()
	};

	// println!("++++++abi:{}", abi);
	fs::write(Path::new("./target/testabi.abi"), abi)?;
	Ok(())
}
