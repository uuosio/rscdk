use std::thread;

use eosio_chaintester::client;

fn main() {
    for i in 0..10 {
        match client::run(9090) {
            Ok(()) => println!("chaintester client ran successfully"),
            Err(e) => {
                println!("chaintester client failed with error {:?}", e);
                std::process::exit(1);
            }
        }
    }
}
