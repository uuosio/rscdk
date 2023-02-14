use crate::structs::*;
use crate::vmapi;
use crate::name::{ Name };

///
pub fn prints(s: &str) {
    vmapi::print::prints_l(s.as_bytes().as_ptr(), s.len() as u32);
}

///
pub fn printi(value: i64) {
    vmapi::print::printi(value);
}

///
pub fn printui(value: u64) {
    vmapi::print::printui(value);
}

///
pub fn printi128(value: i128) {
    vmapi::print::printi128(value);
}

///
pub fn printui128(value: u128) {
    vmapi::print::printui128(value);
}

///
pub fn printsf(value: f32) {
    vmapi::print::printsf(value);
}

///
pub fn printdf(value: f64) {
    vmapi::print::printdf(value);
}

///
pub fn printqf(value: &Float128) {
    vmapi::print::printqf(value);
}

///
pub fn printn(name: Name) {
    vmapi::print::printn(name.value());
}

///
pub fn printhex(data: &[u8]) {
    vmapi::print::printhex(data.as_ptr(), data.len() as u32);
}

///
pub trait Printable {
    ///
    fn print(&self);
}

impl Printable for Name {
    ///
    fn print(&self) {
        printn(*self);
    }
}

impl Printable for bool {
    ///
    fn print(&self) {
        if *self {
            prints("true");
        } else {
            prints("false");
        }
    }
}

impl Printable for i8 {
    ///
    fn print(&self) {
        printi(*self as i64);
    }
}

impl Printable for u8 {
    ///
    fn print(&self) {
        printi(*self as i64);
    }
}

impl Printable for i16 {
    ///
    fn print(&self) {
        printi(*self as i64);
    }
}

impl Printable for u16 {
    ///
    fn print(&self) {
        printi(*self as i64);
    }
}

impl Printable for i32 {
    ///
    fn print(&self) {
        printi(*self as i64);
    }
}

impl Printable for u32 {
    ///
    fn print(&self) {
        printui(*self as u64);
    }
}

impl Printable for usize {
    ///
    fn print(&self) {
        printui(*self as u64);
    }
}

impl Printable for i64 {
    ///
    fn print(&self) {
        printi(*self);
    }
}

impl Printable for u64 {
    ///
    fn print(&self) {
        printui(*self);
    }
}

impl Printable for i128 {
    ///
    fn print(&self) {
        printi128(*self);
    }
}

impl Printable for u128 {
    ///
    fn print(&self) {
        printui128(*self);
    }
}

impl Printable for [u8] {
    ///
    fn print(&self) {
        printhex(self);
    }
}

impl Printable for str {
    ///
    fn print(&self) {
        prints(self);
    }
}

impl Printable for f32 {
    ///
    fn print(&self) {
        printsf(*self);
    }
}

impl Printable for f64 {
    ///
    fn print(&self) {
        printdf(*self);
    }
}

impl Printable for Float128 {
    ///
    fn print(&self) {
        printqf(self);
    }
}

// #[macro_export]
// macro_rules! eosio_print {
//     ( $( $x:expr ),* ) => {
//         {
//             $(
//                 $x.print();
//                 prints(" ");
//             )*
//         }
//     };
// }

///
#[macro_export]
macro_rules! eosio_print {
    ($last:expr) => {
        $last.print();
    };
    ($head:expr, $($tail:expr), +) => {
        $head.print();
        $crate::print::prints(" ");
        eosio_print!($($tail),+);
    };
}

///
#[macro_export]
macro_rules! eosio_println {
    ( $last:expr ) => {
        $last.print();
        $crate::print::prints("\n");
    };
    ( $head:expr, $($tail:expr), + ) => {
        $head.print();
        $crate::print::prints(" ");
        eosio_println!($($tail),+);
    };
}
