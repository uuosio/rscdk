use core::ops;

use crate::serializer::{
    Packer,
    Encoder,
    Decoder,
};

use crate::name::{
    Name,
};

use crate::print::{
    Printable,
    prints,
};

use crate::vec::Vec;
use crate::string::{
    String,
    ToString,
};

use crate::vmapi::eosio::{
    check,
};

const MAX_AMOUNT: i64 = (1 << 62) - 1;

pub fn is_valid_symbol_code(sym: u64) -> bool {
    let mut i: i32 = 0;
    let mut tmp = sym;

    for j in 0..7 {
        let c = (tmp & 0xFF) as u8;
        if !(c >= 'A' as u8 && c <= 'Z' as u8) {
            return false;
        }

        tmp >>= 8;
        if (tmp & 0xFF) == 0 {
            break;
        }
        i = j;
    }
    i += 1;

    for _ in i..7 {
        tmp >>= 8;
        if (tmp & 0xFF) != 0 {
            return false;
        }
    }
    return true;
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct SymbolCode {
    ///
    value: u64,
}

impl SymbolCode {
    ///
    pub fn new(value: u64) -> Self {
        check(is_valid_symbol_code(value), "bad symbol value");
        Self{value}
    }

    ///
    pub fn value(&self) -> u64 {
        self.value
    }

    ///
    pub fn to_string(&self) -> String {
        let mut v: Vec<u8> = Vec::with_capacity(7);
        let mut tmp = self.value;
        for _ in 0..7 {
            v.push((tmp & 0xff) as u8);
            tmp >>= 8;
            if tmp <= 0 {
                break;
            }
        }
        String::from_utf8(v).unwrap()
    }

    ///
    pub fn is_valid(&self) -> bool {
        return is_valid_symbol_code(self.value);
    }
}

impl Packer for SymbolCode {
    ///
    fn size(&self) -> usize {
        return 8;
    }

    ///
    fn pack(&self, enc: &mut Encoder) -> usize {
        self.value.pack(enc)
    }

    ///
    fn unpack(&mut self, data: &[u8]) -> usize {
        check(data.len() >= self.size(), "SymbolCode.unpack: buffer overflow");
        self.value.unpack(data);
        check(self.is_valid(), "SymbolCode.unpack:: bad symbol code");
        return 8;
    }
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Symbol {
    ///
    value: u64,
}

impl Symbol {
    ///
    pub fn new(name: &str, precision: u8) -> Self {
        let raw = name.as_bytes();
        check(raw.len() < 7, "bad symbol name");

        let mut value: u64 = 0;
        for i in (0..raw.len()).rev() {
            let c = raw[i];
            check(c >= 'A' as u8 && c <= 'Z' as u8, "invald symbol character");
            value <<= 8;
            value |= c as u64;
        }

        value <<= 8;
        value |= precision as u64;
        Self{value}
    }

    ///
    pub fn value(&self) -> u64 {
        self.value
    }

    ///
    pub fn code(&self) -> SymbolCode {
        SymbolCode{value: self.value >> 8}
    }

    ///
    pub fn precision(&self) -> usize {
        (self.value & 0xFF) as usize
    }

    ///
    pub fn to_string(&self) -> String {
        self.precision().to_string() + "," + &self.code().to_string()
    }

    ///
    pub fn is_valid(&self) -> bool {
        return self.code().is_valid();
    }
}

impl Packer for Symbol {
    ///
    fn size(&self) -> usize {
        return 8;
    }

    ///
    fn pack(&self, enc: &mut Encoder) -> usize {
        self.value.pack(enc)
    }

    ///
    fn unpack(&mut self, data: &[u8]) -> usize {
        check(data.len() >= self.size(), "Symbol.unpack: buffer overflow");
        self.value.unpack(data);
        check(self.code().is_valid(), "Symbol.unpack: bad symbol value");
        return 8;
    }
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Asset {
    ///
    amount: i64,
    ///
    symbol: Symbol,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum AssetStringParseStatus {
    Initial,
    FoundDot,
    FoundSpace,
}

fn is_amount_within_range(amount: i64) -> bool {
    return -MAX_AMOUNT <= amount && amount <= MAX_AMOUNT;
}

impl Asset {
    ///
    pub fn new(amount: i64, symbol: Symbol) -> Self {
        check(is_amount_within_range(amount), "bad amount value");
        check(symbol.is_valid(), "invalid symbol name");
        Self{amount, symbol}
    }

    ///
    pub fn from_string(s: &str) -> Self {
        let mut status = AssetStringParseStatus::Initial;
        let raw = s.as_bytes();

        let mut amount: i64 = 0;
        let mut symbol: u64 = 0;
        let mut precision: u8 = 0;
        let mut raw_symbol: Vec<u8> = Vec::with_capacity(7);

        for &c in raw {
            if c == '.' as u8 {
                check(status == AssetStringParseStatus::Initial, "Asset.from_string: invalid dot character");
                status = AssetStringParseStatus::FoundDot;
                continue;
            } else if c == ' ' as u8 {
                check(status == AssetStringParseStatus::Initial || status == AssetStringParseStatus::FoundDot, "Asset.from_string: invalid space character");
                // if status == AssetStringParseStatus::FoundDot {
                //     check(precision > 0, "Asset.from_string: invalid precision");
                // }
                status = AssetStringParseStatus::FoundSpace;
                continue;
            }

            match status {
                AssetStringParseStatus::Initial => {
                    check(c >= '0' as u8 && c <= '9' as u8, "Asset.from_string: bad amount");
                    amount *= 10;
                    amount += (c - '0' as u8) as i64;
                }
                AssetStringParseStatus::FoundDot => {
                    check(c >= '0' as u8 && c <= '9' as u8, "Asset.from_string: bad amount");
                    amount *= 10;
                    amount += (c - '0' as u8) as i64;
                    precision += 1;                        
                }
                AssetStringParseStatus::FoundSpace => {
                    check(c >= 'A' as u8 && c <= 'Z' as u8, "Asset.from_string: bad symbol");
                    raw_symbol.push(c);
                }
            }
        }

        raw_symbol.reverse();
        for c in raw_symbol {
            symbol <<= 8;
            symbol |= c as u64;
        }

        symbol <<= 8;
        symbol |= precision as u64;

        Self{
            amount: amount,
            symbol: Symbol{value: symbol}
        }
    }

    ///
    pub fn amount(&self) -> i64 {
        self.amount
    }

    ///
    pub fn symbol(&self) -> Symbol {
        self.symbol
    }

    ///
    pub fn to_string(self) -> String {
        let mut part1: i64 = self.amount;

        for _ in 0..self.symbol.precision() {
            part1 /= 10;
        }

        let mut part2:Vec<u8> = Vec::with_capacity(self.symbol.precision());
        part2.resize(self.symbol.precision(), 0u8);

        let mut tmp: i64 = self.amount;
        for i in (0..self.symbol.precision()).rev() {
            part2[i] = '0' as u8 + (tmp % 10) as u8;
            tmp /= 10;
        }
        part1.to_string() + "." + &String::from_utf8(part2).unwrap() + " " + &self.symbol.code().to_string()
    }

    ///
    pub fn is_valid(&self) -> bool {
        return is_amount_within_range(self.amount) && self.symbol().is_valid();
    }
}

// check(a.symbol.value == b.symbol.value, "symbol not the same");
// let amount: i64 = a.amount + b.amount;
// check(-MAX_AMOUNT <= amount, "addition underflow");
// check(amount <= MAX_AMOUNT, "addition overflow");
// return new Asset(amount, Symbol.fromU64(a.symbol.value));

impl ops::Add for Asset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        check(self.symbol == rhs.symbol, "add: bad symbol");
        let amount = self.amount + rhs.amount;
        check(amount >= -MAX_AMOUNT, "addition underflow");
        check(amount <= MAX_AMOUNT, "addition overflow");
        Self {
            amount: amount,
            symbol: self.symbol
        }
    }
}

impl ops::AddAssign for Asset {
    fn add_assign(&mut self, rhs: Asset) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Asset {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        check(self.symbol == rhs.symbol, "sub: bad symbol");
        let amount = self.amount() - rhs.amount();
        check(amount >= -MAX_AMOUNT, "subtraction underflow");
        check(amount <= MAX_AMOUNT, "subtraction overflow");
        Self {
            amount: amount,
            symbol: self.symbol
        }
    }
}

impl ops::SubAssign for Asset {
    fn sub_assign(&mut self, rhs: Asset) {
        *self = *self - rhs;
    }
}

impl Printable for Asset {
    fn print(&self) {
        prints(&self.to_string());
    }
}

impl Packer for Asset {
    ///
    fn size(&self) -> usize {
        return 16;
    }

    ///
    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();

        self.amount.pack(enc);
        self.symbol.pack(enc);

        enc.get_size() - pos
    }

    ///
    fn unpack(&mut self, data: &[u8]) -> usize {
        check(data.len() >= self.size(), "Asset.unpack: buffer overflow");

        let mut dec = Decoder::new(data);
        dec.unpack(&mut self.amount);
        check(self.amount >= -MAX_AMOUNT && self.amount <= MAX_AMOUNT, "Asset.unpack: bad asset amount");
        dec.unpack(&mut self.symbol);
        dec.get_pos()
    }
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct ExtendedAsset {
    ///
	quantity: Asset,
    ///
	contract: Name,
}

impl ExtendedAsset {
    ///
    pub fn new(quantity: Asset, contract: Name) -> Self {
        Self{quantity, contract}
    }

    ///
    pub fn quantity(&self) -> Asset {
        self.quantity
    }

    ///
    pub fn contract(&self) -> Name {
        self.contract
    }
}

impl Packer for ExtendedAsset {
    ///
    fn size(&self) -> usize {
        return 16 + 8;
    }

    ///
    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();

        self.quantity.pack(enc);
        self.contract.pack(enc);

        enc.get_size() - pos
    }

    ///
    fn unpack(&mut self, data: &[u8]) -> usize {
        check(data.len() >= self.size(), "ExtendedAsset.unpack: buffer overflow");

        let mut dec = Decoder::new(data);
        dec.unpack(&mut self.quantity);
        dec.unpack(&mut self.contract);
        dec.get_pos()
    }
}
