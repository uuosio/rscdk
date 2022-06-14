use std::{fs, path::Path};

extern crate contract;

extern "Rust" {
	fn __eosio_generate_abi() -> eosio_metadata::MetadataVersioned;
}

fn main() -> Result<(), std::io::Error> {
	let metadata = unsafe { __eosio_generate_abi() };
	if let eosio_metadata::MetadataVersioned::V3(prj) = metadata {
		let contents = serde_json::to_string_pretty(&prj)?;
		// print!("{}", contents);
		fs::write(Path::new("./target/hello.abi"), contents)?;	
	}
	Ok(())
}
