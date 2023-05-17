extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Result of add: {}", add(2, 3));
    }
}
