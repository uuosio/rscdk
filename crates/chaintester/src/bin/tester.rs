use std::thread;

use chaintester::client;

fn main() {
    for i in 0..10 {
        match client::run(9090, i) {
            Ok(()) => println!("chaintester client ran successfully"),
            Err(e) => {
                println!("chaintester client failed with error {:?}", e);
                std::process::exit(1);
            }
        }    
    }
}
