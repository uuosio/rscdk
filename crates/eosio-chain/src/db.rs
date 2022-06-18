use crate::structs::{ Uint128, Uint256, Float128, Checksum256 };
use crate::serializer::Packer;
use crate::print;

use crate::vmapi::db::*;

use crate::vmapi::eosio::{
    eosio_memcpy,
};

use crate::asset::Asset;

use crate::{
    check,
    eosio_assert,
};

use crate::print::{
    Printable
};

use crate::{
    vec,
    vec::Vec,
};

use crate::string::{
    String,
};

///
pub trait ToPrimaryValue {
    ///
    fn to_primary_value(&self) -> u64;
}

///
pub trait ToSecondaryValue {
    ///
    fn to_secondary_value(&self, idx_type: SecondaryType) -> SecondaryValue;
}


///
#[derive(Clone, Debug, Default)]
pub struct DBError {
    ///
    pub message: String,
}

///
pub trait FromSecondaryValue: Sized {
    ///
    fn from_secondary_value(value: SecondaryValue) -> Self;
}

///
pub trait TryFromSecondaryValue: Sized {
    ///
    type Error;
    ///
    fn from_secondary_value(value: SecondaryValue) -> Result<Self, Self::Error>;
}

impl ToPrimaryValue for u64 {
    fn to_primary_value(&self) -> u64 {
        return *self;
    }
}

impl ToSecondaryValue for u64 {
    fn to_secondary_value(&self, idx_type: SecondaryType) -> SecondaryValue {
        if idx_type == SecondaryType::Idx64 {
            return SecondaryValue::Idx64(*self);
        } else {
            check(false, "u64.to_secondary_value: unsupported SecondaryType");
            return SecondaryValue::None;
        }
    }
}

impl FromSecondaryValue for u64 {
    fn from_secondary_value(value: SecondaryValue) -> u64 {
        if let SecondaryValue::Idx64(x) = value {
            x
        } else {
            eosio_assert(false, "from value is not an Idx64 value");
            0
        }
    }
}

impl TryFromSecondaryValue for u64 {
    ///
    type Error = DBError;
    ///
    fn from_secondary_value(value: SecondaryValue) -> Result<Self, Self::Error> {
        if let SecondaryValue::Idx64(x) = value {
            Ok(x)
        } else {
            Err(Self::Error{message: String::from("from value is not an Idx64 value")})
        }
    }
}


impl ToPrimaryValue for i64 {
    fn to_primary_value(&self) -> u64 {
        return *self as u64;
    }
}

impl ToSecondaryValue for i64 {
    fn to_secondary_value(&self, idx_type: SecondaryType) -> SecondaryValue {
        if idx_type == SecondaryType::Idx64 {
            return SecondaryValue::Idx64(*self as u64);
        }
        check(false, "ToSecondaryValue for i64: invalid idx_type");
        return SecondaryValue::None;
    }
}

impl FromSecondaryValue for i64 {
    fn from_secondary_value(value: SecondaryValue) -> i64 {
        if let SecondaryValue::Idx64(x) = value {
            x as i64
        } else {
            eosio_assert(false, "from value is not an Idx64 value");
            0
        }
    }
}

impl TryFromSecondaryValue for i64 {
    ///
    type Error = DBError;
    ///
    fn from_secondary_value(value: SecondaryValue) -> Result<Self, Self::Error> {
        if let SecondaryValue::Idx64(x) = value {
            Ok(x as i64)
        } else {
            Err(Self::Error{message: String::from("from value is not an Idx64 value")})
        }
    }
}

impl ToSecondaryValue for f64 {
    fn to_secondary_value(&self, idx_type: SecondaryType) -> SecondaryValue {
        if SecondaryType::IdxF64 == idx_type {
            return SecondaryValue::IdxF64(*self);
        } else {
            check(false, "f64:to_secondary_value: unsupported secondary type");
            return SecondaryValue::None;
        }
    }
}

impl FromSecondaryValue for f64 {
    fn from_secondary_value(value: SecondaryValue) -> f64 {
        if let SecondaryValue::IdxF64(x) = value {
            x
        } else {
            eosio_assert(false, "from value is not an Idx64 value");
            0.0
        }
    }
}


impl ToSecondaryValue for Float128 {
    fn to_secondary_value(&self, idx_type: SecondaryType) -> SecondaryValue {
        if SecondaryType::IdxF128 == idx_type {
            return SecondaryValue::IdxF128(*self);
        } else {
            check(false, "f64:to_secondary_value: unsupported secondary type");
            return SecondaryValue::None;
        }
    }
}

impl FromSecondaryValue for Float128 {
    fn from_secondary_value(value: SecondaryValue) -> Float128 {
        if let SecondaryValue::IdxF128(x) = value {
            x
        } else {
            eosio_assert(false, "from value is not an Idx64 value");
            Float128::default()
        }
    }
}

impl ToSecondaryValue for u128 {
    fn to_secondary_value(&self, idx_type: SecondaryType) -> SecondaryValue {
        if idx_type == SecondaryType::Idx128 {
            return SecondaryValue::Idx128(*self);
        }
        check(false, "ToSecondaryValue for u128: invalid idx_type");
        return SecondaryValue::None;
    }
}

impl FromSecondaryValue for u128 {
    fn from_secondary_value(value: SecondaryValue) -> u128 {
        if let SecondaryValue::Idx128(x) = value {
            x
        } else {
            check(false, "from value is not an Idx64 value");
            0
        }
    }
}

impl FromSecondaryValue for Checksum256 {
    fn from_secondary_value(value: SecondaryValue) -> Checksum256 {
        if let SecondaryValue::Idx256(x) = value {
            let mut ret: Checksum256 = Default::default();
            eosio_memcpy(ret.data.as_mut_ptr(), x.data.as_ptr() as *const u8, 32);
            return ret;
        } else {
            check(false, "FromSecondaryValue<Checksum256>: bad secondary value");
            return Default::default();
        }
    }
}

impl ToSecondaryValue for Checksum256 {
    fn to_secondary_value(&self, idx_type: SecondaryType) -> SecondaryValue {
        if SecondaryType::Idx256 == idx_type {
            let mut ret = Uint256::default();
            eosio_memcpy(ret.data.as_mut_ptr() as *mut u8, self.data.as_ptr() as *const u8, 32);
            return SecondaryValue::Idx256(ret);
        } else {
            check(false, "ToSecondaryValue: bad Secondary type!");
            return SecondaryValue::Idx256(Uint256::default());
        }
    }
}

impl FromSecondaryValue for Uint256 {
    fn from_secondary_value(value: SecondaryValue) -> Uint256 {
        if let SecondaryValue::Idx256(x) = value {
            let mut ret: Uint256 = Default::default();
            eosio_memcpy(ret.data.as_mut_ptr() as *mut u8, x.data.as_ptr() as *const u8, 32);
            return ret;
        } else {
            check(false, "FromSecondaryValue<Checksum256>: bad secondary value");
            return Default::default();
        }
    }
}

impl ToSecondaryValue for Uint256 {
    fn to_secondary_value(&self, idx_type: SecondaryType) -> SecondaryValue {
        if SecondaryType::Idx256 == idx_type {
            let mut ret = Uint256::default();
            eosio_memcpy(ret.data.as_mut_ptr() as *mut u8, self.data.as_ptr() as *const u8, 32);
            return SecondaryValue::Idx256(ret);
        } else {
            check(false, "ToSecondaryValue: bad Secondary type!");
            return SecondaryValue::Idx256(Uint256::default());
        }
    }
}

impl ToPrimaryValue for Asset {
    fn to_primary_value(&self) -> u64 {
        return self.symbol().code().value();
    }
}

///
pub trait DBInterface {
    ///
    fn get_primary(&self) -> u64;
    ///
    fn get_secondary_value(&self, i: usize) -> SecondaryValue;
    ///
    fn set_secondary_value(&mut self, i: usize, value: SecondaryValue);
}

///
pub trait MultiIndexValue: DBInterface + Packer {

}

///
pub trait DBValue {
    ///
	fn get_primary(&self) -> u64;
    ///
	fn pack(&self) -> Vec<u8>;
    ///
	fn unpack(data: &[u8]) -> Self;
}


///
pub struct IdxTable {
    ///
    pub code: u64,
    ///
    pub scope: u64,
    ///
    pub table: u64,
}

///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Iterator {
    ///
    pub i: i32,
}

impl Iterator {
    ///
    pub fn is_ok(&self) -> bool {
        self.i >= 0
    }
    ///
    pub fn is_end(&self) -> bool {
        return self.i == -2;
    }
}

///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecondaryIterator {
    ///
    pub i: i32,
    ///
    pub db_index: usize,
    ///
    pub primary: u64
}

impl SecondaryIterator {
    ///
    pub fn is_ok(&self) -> bool {
        self.i >= 0
    }
    ///
    pub fn is_end(&self) -> bool {
        return self.i == -2;
    }
}

///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecondaryType {
    ///
    Idx64,
    ///
    Idx128,
    ///
    Idx256,
    ///
    IdxF64,
    ///
    IdxF128,
}

///
#[derive(Clone, Copy, PartialEq)]
pub enum SecondaryValue {
    ///
    None,
    ///
    Idx64(u64),
    ///
    Idx128(u128),
    ///
    Idx256(Uint256),
    ///
    IdxF64(f64),
    ///
    IdxF128(Float128),
}

///
pub struct DBI64 {
    ///
    pub code: u64,
    ///
    pub scope: u64,
    ///
    pub table: u64,
}

impl DBI64 {
    ///
    pub fn new(code: u64, scope: u64, table: u64) -> Self {
        DBI64 {
            code,
            scope,
            table,
        }
    }

    ///
    pub fn store(&self, payer: u64, id: u64,  data: &[u8]) -> Iterator {
        let it = db_store_i64(self.scope, self.table, payer, id, data.as_ptr(), data.len() as u32);
        Iterator { i: it }
    }

    ///
    pub fn update(&self, iterator: Iterator, data: &[u8], payer: u64) {
        db_update_i64(iterator.i, payer, data.as_ptr(), data.len() as u32);
    }

    ///
    pub fn remove(&self, iterator: Iterator) {
        db_remove_i64(iterator.i);
    }

    ///
    pub fn get(&self, iterator: Iterator) -> Vec<u8> {
        //, data: *const u32, len: u32
        let size = db_get_i64(iterator.i, 0 as *const u8, 0);
        if size == 0 {
            return Vec::new();
        }
        let mut data: Vec<u8> = vec![0; size as usize];
        // let mut data: Vec<u8> = Vec::with_capacity(size as usize);
        // data.resize_with(size as usize, Default::default);
        let ptr = data.as_mut_ptr();
        db_get_i64(iterator.i, ptr, size as u32);
        return data;
    }

    ///
    pub fn next(&self, iterator: Iterator) -> (Iterator, u64) {
        let mut primary = 0;
        let it = db_next_i64(iterator.i, &mut primary);
        (Iterator { i: it }, primary)
    }

    ///
    pub fn previous(&self, iterator: Iterator) -> (Iterator, u64) {
        let mut primary = 0;
        let it = db_previous_i64(iterator.i, &mut primary);
        (Iterator { i: it ,}, primary)
    }

    ///
    pub fn find(&self, id: u64) -> Iterator {
        let it = db_find_i64(self.code, self.scope, self.table, id);
        Iterator { i: it }
    }

    ///
    pub fn lowerbound(&self, id: u64) -> Iterator {
        let it = db_lowerbound_i64(self.code, self.scope, self.table, id);
        Iterator { i: it }
    }

    ///
    pub fn upperbound(&self, id: u64) -> Iterator {
        let it = db_upperbound_i64(self.code, self.scope, self.table, id);
        Iterator { i: it }
    }

    ///
    pub fn end(&self) -> Iterator {
        let it = db_end_i64(self.code, self.scope, self.table);
        Iterator { i: it }
    }
}

///
pub struct Idx64DB {
    ///
    pub db_index: usize,
    ///
    pub code: u64,
    ///
    pub scope: u64,
    ///
    pub table: u64,
}

///
pub struct Idx128DB {
    ///
    pub db_index: usize,
    ///
    pub code: u64,
    ///
    pub scope: u64,
    ///
    pub table: u64,
}

///
pub struct Idx256DB {
    ///
    pub db_index: usize,
    ///
    pub code: u64,
    ///
    pub scope: u64,
    ///
    pub table: u64,
}

///
pub struct IdxF64DB {
    ///
    pub db_index: usize,
    ///
    pub code: u64,
    ///
    pub scope: u64,
    ///
    pub table: u64,
}

///
pub struct IdxF128DB {
    ///
    pub db_index: usize,
    ///
    pub code: u64,
    ///
    pub scope: u64,
    ///
    pub table: u64,
}

///
pub trait IndexDB {
    // fn new(code: u64, scope: u64, table: u64) -> Self;
    ///
    fn get_db_index(&self) -> usize;
    ///
    fn store(&self, payer: u64, id: u64, secondary: SecondaryValue) -> SecondaryIterator;
    ///
    fn update(&self, iterator: SecondaryIterator, secondary: SecondaryValue, payer: u64);
    ///
    fn remove(&self, iterator: SecondaryIterator);
    ///
    fn next(&self, iterator: SecondaryIterator) -> SecondaryIterator;
    ///
    fn previous(&self, iterator: SecondaryIterator) -> SecondaryIterator;
    ///
    fn find_primary(&self, primary: u64) -> (SecondaryIterator, SecondaryValue);
    ///
    fn find(&self, secondary: SecondaryValue) -> SecondaryIterator;
    ///
    fn lowerbound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue);
    ///
    fn upperbound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue);
    ///
    fn end(&self) -> SecondaryIterator;
}

///
pub struct IndexDBProxy<'a, T: ToSecondaryValue + FromSecondaryValue + Printable + Default, const IDX_TYPE: usize> {
    ///
    pub db: &'a dyn IndexDB,
    __: T,
    secondary_type: SecondaryType,
}

fn index_to_secondary_type(i: usize) -> SecondaryType {
    return match i {
        0 => SecondaryType::Idx64,
        1 => SecondaryType::Idx128,
        2 => SecondaryType::Idx256,
        3 => SecondaryType::IdxF64,
        4 => SecondaryType::IdxF128,
        _ => SecondaryType::Idx64,
    }
}

impl<'a, T: ToSecondaryValue + FromSecondaryValue + Printable + Default, const IDX_TYPE: usize> IndexDBProxy<'a, T, IDX_TYPE> {
    ///
    pub fn new(db: &'a dyn IndexDB) -> Self {
        Self {
            db,
            __: Default::default(),
            secondary_type: index_to_secondary_type(IDX_TYPE),
        }
    }
    ///
    pub fn get_db_index(&self) -> usize {
        return 0;
    }

    ///
    pub fn store(&self, payer: u64, id: u64, value: T) -> SecondaryIterator {
        return self.db.store(payer, id, value.to_secondary_value(self.secondary_type));
    }

    ///
    pub fn update(&self, iterator: SecondaryIterator, value: T, payer: u64) {
        self.db.update(iterator, value.to_secondary_value(self.secondary_type), payer);
    }

    ///
    pub fn remove(&self, iterator: SecondaryIterator) {
        self.db.remove(iterator);
    }

    ///
    pub fn next(&self, iterator: SecondaryIterator) -> SecondaryIterator {
        return self.db.next(iterator);
    }

    ///
    pub fn previous(&self, iterator: SecondaryIterator) -> SecondaryIterator {
        return self.db.previous(iterator);
    }

    ///
    pub fn find_primary(&self, primary: u64) -> (SecondaryIterator, T) {
        let (it, value) = self.db.find_primary(primary);
        return (it, T::from_secondary_value(value));
    }

    ///
    pub fn find(&self, secondary: T) -> SecondaryIterator {
        return self.db.find(secondary.to_secondary_value(self.secondary_type));
    }

    ///
    pub fn lowerbound(&self, secondary: T) -> (SecondaryIterator, T) {
        let (it, value) = self.db.lowerbound(secondary.to_secondary_value(self.secondary_type));
        let _secondary: T = Default::default();
        return (it, T::from_secondary_value(value));
    }

    ///
    pub fn upperbound(&self, secondary: T) -> (SecondaryIterator, T) {
        let _secondary = secondary.to_secondary_value(self.secondary_type);
        let (it, value) = self.db.upperbound(_secondary);
        let _value = T::from_secondary_value(value);
        return (it, _value);
    }

    ///
    pub fn end(&self) -> SecondaryIterator {
        return self.db.end();
    }

}

impl Idx64DB {
    ///
    pub fn new(db_index: usize, code: u64, scope: u64, table: u64) -> Self {
        Idx64DB { db_index: db_index, code, scope, table }
    }
}

impl IndexDB for Idx64DB {
    fn get_db_index(&self) -> usize {
        return self.db_index;
    }

    fn store(&self, payer: u64, id: u64, secondary: SecondaryValue) -> SecondaryIterator {
        if let SecondaryValue::Idx64(value) = secondary {
            print::printui(value);
            let ret = db_idx64_store(self.scope, self.table, payer, id, &value);
            return SecondaryIterator{ i: ret, primary: id, db_index: self.db_index };    
        }
        check(false, "Idx64DB::store: bad secondary type");
        return SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index }
    }

    fn update(&self, iterator: SecondaryIterator, secondary: SecondaryValue, payer: u64) {
        if let SecondaryValue::Idx64(value) = secondary {
            db_idx64_update(iterator.i, payer, &value);
            return;
        } else {
            check(false, "Idx64DB::update: bad secondary type");
            return;
        }
    }

    fn remove(&self, iterator: SecondaryIterator) {
        db_idx64_remove(iterator.i);
    }

    fn next(&self, iterator: SecondaryIterator) -> SecondaryIterator {
        let mut primary = 0;
        let ret = db_idx64_next(iterator.i, &mut primary);
        return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
    }

    fn previous(&self, iterator: SecondaryIterator) -> SecondaryIterator {
        let mut primary = 0;
        let ret = db_idx64_previous(iterator.i, &mut primary);
        return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
    }

    fn find_primary(&self, primary: u64) -> (SecondaryIterator, SecondaryValue) {
        let mut secondary: u64 = 0;
        let ret = db_idx64_find_primary(self.code, self.scope, self.table, &mut secondary, primary);
        return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::Idx64(secondary));
    }

    fn find(&self, secondary: SecondaryValue) -> SecondaryIterator {
        if let SecondaryValue::Idx64(value) = secondary {
            let mut primary = 0;
            let ret = db_idx64_find_secondary(self.code, self.scope, self.table, &value, &mut primary);
            return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
        }
        check(false, "Idx64DB::find_secondary: bad secondary type");
        return SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index };
    }

    fn lowerbound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        if let SecondaryValue::Idx64(mut value) = secondary {
            let mut primary = 0;
            let ret = db_idx64_lowerbound(self.code, self.scope, self.table, &mut value, &mut primary);
            
            return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::Idx64(value));    
        }
        check(false, "Idx64DB::lowerbound: bad secondary type");
        return (SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index }, SecondaryValue::Idx64(0));
    }

    fn upperbound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        if let SecondaryValue::Idx64(mut value) = secondary {
            let mut primary = 0;
            let ret = db_idx64_upperbound(self.code, self.scope, self.table, &mut value, &mut primary);            
            return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::Idx64(value));
        }
        check(false, "Idx64DB::upperbound: bad secondary type");
        return (SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index }, SecondaryValue::Idx128(0));
    }

    fn end(&self) -> SecondaryIterator {
        let ret = db_idx64_end(self.code, self.scope, self.table);
        return SecondaryIterator{ i: ret, primary: 0, db_index: self.db_index };
    }
}

impl Idx128DB {
    ///
    pub fn new(db_index: usize, code: u64, scope: u64, table: u64) -> Self {
        Idx128DB {db_index: db_index, code, scope, table }
    }
}

impl IndexDB for Idx128DB {
    fn get_db_index(&self) -> usize {
        return self.db_index;
    }

    fn store(&self, payer: u64, id: u64, secondary: SecondaryValue) -> SecondaryIterator {
        if let SecondaryValue::Idx128(value) = secondary {
            let _secondary = Uint128{lo: (value & 0xffffffffffffffff) as u64, hi: (value >> 64) as u64};
            let ret = db_idx128_store(self.scope, self.table, payer, id, &_secondary);
            return SecondaryIterator{ i: ret, primary: id, db_index: self.db_index };
        }
        check(false, "Idx128DB::store: bad secondary type");
        return SecondaryIterator{ i: -1, primary: 0, db_index: 0 };
        //return SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index };
    }

    fn update(&self, iterator: SecondaryIterator, secondary: SecondaryValue, payer: u64) {
        if let SecondaryValue::Idx128(value) = secondary {
            let _secondary = Uint128{lo: (value & 0xffffffffffffffff) as u64, hi: (value >> 64) as u64};
            db_idx128_update(iterator.i, payer, &_secondary);
        } else {
            check(false, "Idx128DB::update: bad secondary type");
        }
    }

    fn remove(&self, iterator: SecondaryIterator) {
        db_idx128_remove(iterator.i);
    }

    fn next(&self, iterator: SecondaryIterator) -> SecondaryIterator {
        let mut primary = 0;
        let ret = db_idx128_next(iterator.i, &mut primary);
        return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
    }

    fn previous(&self, iterator: SecondaryIterator) -> SecondaryIterator {
        let mut primary = 0;
        let ret = db_idx128_previous(iterator.i, &mut primary);
        return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
    }

    fn find_primary(&self, primary: u64) -> (SecondaryIterator, SecondaryValue) {
        //initialize Uint128
        let mut secondary = Uint128{lo:0, hi: 0};
        let ret = db_idx128_find_primary(self.code, self.scope, self.table, &mut secondary, primary);
        let _secondary = (secondary.hi as u128) << 64 + secondary.lo as u128;
        return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::Idx128(_secondary));
    }

    fn find(&self, secondary: SecondaryValue) -> SecondaryIterator {
        if let SecondaryValue::Idx128(value) = secondary {
            let mut primary = 0;
            let mut _secondary = Uint128{lo: (value & 0xffffffffffffffff) as u64, hi: (value >> 64) as u64};
            let ret = db_idx128_find_secondary(self.code, self.scope, self.table, &mut _secondary, &mut primary);
            return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
        }
        check(false, "Idx128DB::find_secondary: bad secondary type");
        return SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index };
    }

    fn lowerbound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        if let SecondaryValue::Idx128(mut value) = secondary {
            let mut primary = 0;
            // let _secondary: SecondaryValue = secondary;
            let mut _secondary = Uint128{lo: (value & 0xffffffffffffffff) as u64, hi: (value >> 64) as u64};
            let ret = db_idx128_lowerbound(self.code, self.scope, self.table, &mut _secondary, &mut primary);
            value = (_secondary.hi as u128) << 64 + _secondary.lo as u128;
            return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::Idx128(value));
        }
        check(false, "Idx128DB::lowerbound: bad secondary type");
        return (SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index }, SecondaryValue::Idx128(0));
    }

    fn upperbound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        match secondary {
            SecondaryValue::Idx128(mut value) => {
                let mut primary = 0;
                // let _secondary = secondary;
                let mut _secondary = Uint128{lo: (value & 0xffffffffffffffff) as u64, hi: (value >> 64) as u64};
                let ret = db_idx128_upperbound(self.code, self.scope, self.table, &mut _secondary, &mut primary);
                value = (_secondary.hi as u128) << 64 + _secondary.lo as u128;
                return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::Idx128(value));
            },
            _ => {
                check(false, "Idx128DB::upperbound: bad secondary type");
                return (SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index }, SecondaryValue::Idx128(0));
            }
        }
    }

    fn end(&self) -> SecondaryIterator {
        let ret = db_idx128_end(self.code, self.scope, self.table);
        return SecondaryIterator{ i: ret, primary: 0, db_index: self.db_index };
    }
}

impl Idx256DB {
    ///
    pub fn new(db_index: usize, code: u64, scope: u64, table: u64) -> Self {
        Idx256DB {db_index: db_index, code, scope, table }
    }
}

impl IndexDB for Idx256DB {
    fn get_db_index(&self) -> usize {
        return self.db_index;
    }

    fn store(&self, payer: u64, id: u64, secondary: SecondaryValue) -> SecondaryIterator {
        if let SecondaryValue::Idx256(value) = secondary {
            let ret = db_idx256_store(self.scope, self.table, payer, id, value.data.as_ptr() as *mut Uint128, 2);
            return SecondaryIterator{ i: ret, primary: id, db_index: self.db_index };
        }
        check(false, "Idx256DB::store: bad secondary type");
        return SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index };
    }

    fn update(&self, iterator: SecondaryIterator, secondary: SecondaryValue, payer: u64) {
        if let SecondaryValue::Idx256(value) = secondary {
            db_idx256_update(iterator.i, payer, value.data.as_ptr() as *mut Uint128, 2);
        } else {
            check(false, "Idx256DB::update: bad secondary type");
        }
    }

    fn remove(&self, iterator: SecondaryIterator) {
        db_idx256_remove(iterator.i);
    }

    fn next(&self, iterator: SecondaryIterator) -> SecondaryIterator {
        let mut primary = 0;
        let ret = db_idx256_next(iterator.i, &mut primary);
        return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
    }

    fn previous(&self, iterator: SecondaryIterator) -> SecondaryIterator {
        let mut primary = 0;
        let ret = db_idx256_previous(iterator.i, &mut primary);
        return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
    }

    fn find_primary(&self, primary: u64) -> (SecondaryIterator, SecondaryValue) {
        //initialize Uint128
        let mut secondary = Uint256{data: [0; 2]};
        let ret = db_idx256_find_primary(self.code, self.scope, self.table, secondary.data.as_mut_ptr() as *mut Uint128, 2, primary);
        return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::Idx256(secondary));
    }

    fn find(&self, secondary: SecondaryValue) -> SecondaryIterator {
        if let SecondaryValue::Idx256(mut value) = secondary {
            let mut primary = 0;
            let ret = db_idx256_find_secondary(self.code, self.scope, self.table, value.data.as_mut_ptr() as *mut Uint128, 2, &mut primary);
            return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
        }
        check(false, "Idx256DB::find_secondary: bad secondary type");
        return SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index };
    }

    fn lowerbound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        if let SecondaryValue::Idx256(mut value) = secondary {
            let mut primary = 0;
            let _secondary: SecondaryValue = secondary;
            let ret = db_idx256_lowerbound(self.code, self.scope, self.table, value.data.as_mut_ptr() as *mut u8 as *mut Uint128, 2, &mut primary);
            return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, _secondary);
        }
        check(false, "Idx256DB::lowerbound: bad secondary type");
        return (SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index }, SecondaryValue::Idx128(0));
    }

    fn upperbound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        match secondary {
            SecondaryValue::Idx256(mut value) => {
                let mut primary = 0;
                let _secondary = secondary;
                let ret = db_idx256_upperbound(self.code, self.scope, self.table, value.data.as_mut_ptr() as *mut u8 as *mut Uint128, 2, &mut primary);
                return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, _secondary);
            },
            _ => {
                check(false, "Idx256DB::upperbound: bad secondary type");
                return (SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index }, SecondaryValue::Idx128(0));
            }
        }
    }

    fn end(&self) -> SecondaryIterator {
        let ret = db_idx256_end(self.code, self.scope, self.table);
        return SecondaryIterator{ i: ret, primary: 0, db_index: self.db_index };
    }
}


impl IdxF64DB {
    ///
    pub fn new(db_index: usize, code: u64, scope: u64, table: u64) -> Self {
        IdxF64DB { db_index: db_index, code, scope, table }
    }
}

impl IndexDB for IdxF64DB {
    fn get_db_index(&self) -> usize {
        return self.db_index;
    }

    fn store(&self, payer: u64, id: u64, secondary: SecondaryValue) -> SecondaryIterator {
        if let SecondaryValue::IdxF64(value) = secondary {
            let ret = db_idx_double_store(self.scope, self.table, payer, id, &value);
            return SecondaryIterator{ i: ret, primary: id, db_index: self.db_index };
        }
        check(false, "IdxF64DB::store: bad secondary type");
        return SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index };
    }

    fn update(&self, iterator: SecondaryIterator, secondary: SecondaryValue, payer: u64) {
        if let SecondaryValue::IdxF64(value) = secondary {
            db_idx_double_update(iterator.i, payer, &value);
        } else {
            check(false, "IdxF64DB::update: bad secondary type")
        }
    }

    fn remove(&self, iterator: SecondaryIterator) {
        db_idx_double_remove(iterator.i);
    }

    fn next(&self, iterator: SecondaryIterator) -> SecondaryIterator {
        let mut primary = 0;
        let ret = db_idx_double_next(iterator.i, &mut primary);
        return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
    }

    fn previous(&self, iterator: SecondaryIterator) -> SecondaryIterator {
        let mut primary = 0;
        let ret = db_idx_double_previous(iterator.i, &mut primary);
        return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
    }

    fn find_primary(&self, primary: u64) -> (SecondaryIterator, SecondaryValue) {
        //initialize Uint128
        let mut secondary = 0 as f64;
        let ret = db_idx_double_find_primary(self.code, self.scope, self.table, &mut secondary, primary);
        return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::IdxF64(secondary));
    }

    fn find(&self, secondary: SecondaryValue) -> SecondaryIterator {
        if let SecondaryValue::IdxF64(mut value) = secondary {
            let mut primary = 0;
            let ret = db_idx_double_find_secondary(self.code, self.scope, self.table, &mut value, &mut primary);
            return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
        }
        check(false, "IdxF64DB::find_secondary: bad secondary type");
        return SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index };
    }

    fn lowerbound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        if let SecondaryValue::IdxF64(mut value) = secondary {
            let mut primary = 0;
            let _secondary: SecondaryValue = secondary;
            let ret = db_idx_double_lowerbound(self.code, self.scope, self.table, &mut value, &mut primary);
            return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, _secondary);
        }
        check(false, "IdxF64DB::lowerbound: bad secondary type");
        return (SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index }, SecondaryValue::IdxF64(0.0));
    }

    fn upperbound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        match secondary {
            SecondaryValue::IdxF64(mut value) => {
                let mut primary = 0;
                let _secondary = secondary;
                let ret = db_idx_double_upperbound(self.code, self.scope, self.table, &mut value, &mut primary);
                return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, _secondary);    
            },
            _ => {
                check(false, "IdxF64DB::upperbound: bad secondary type");
                return (SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index }, SecondaryValue::IdxF64(0.0));
            }
        }
    }

    fn end(&self) -> SecondaryIterator {
        let ret = db_idx_double_end(self.code, self.scope, self.table);
        return SecondaryIterator{ i: ret, primary: 0, db_index: self.db_index };
    }
}


impl IdxF128DB {
    ///
    pub fn new(db_index: usize, code: u64, scope: u64, table: u64) -> Self {
        IdxF128DB { code, scope, table, db_index: db_index }
    }
}

impl IndexDB for IdxF128DB {
    fn get_db_index(&self) -> usize {
        return self.db_index;
    }

    fn store(&self, payer: u64, id: u64, secondary: SecondaryValue) -> SecondaryIterator {
        if let SecondaryValue::IdxF128(value) = secondary {
            let ret = db_idx_long_double_store(self.scope, self.table, payer, id, &value);
            return SecondaryIterator{ i: ret, primary: id, db_index: self.db_index };
        }
        check(false, "IdxF128DB::store: bad secondary type");
        return SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index };
    }

    fn update(&self, iterator: SecondaryIterator, secondary: SecondaryValue, payer: u64) {
        if let SecondaryValue::IdxF128(value) = secondary {
            db_idx_long_double_update(iterator.i, payer, &value);
        } else {
            check(false, "IdxF128DB::update: bad secondary type")
        }
    }

    fn remove(&self, iterator: SecondaryIterator) {
        db_idx_long_double_remove(iterator.i);
    }

    fn next(&self, iterator: SecondaryIterator) -> SecondaryIterator {
        let mut primary = 0;
        let ret = db_idx_long_double_next(iterator.i, &mut primary);
        return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
    }

    fn previous(&self, iterator: SecondaryIterator) -> SecondaryIterator {
        let mut primary = 0;
        let ret = db_idx_long_double_previous(iterator.i, &mut primary);
        return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
    }

    fn find_primary(&self, primary: u64) -> (SecondaryIterator, SecondaryValue) {
        //initialize Uint128
        let mut secondary: Float128 = Default::default();
        let ret = db_idx_long_double_find_primary(self.code, self.scope, self.table, &mut secondary, primary);
        return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::IdxF128(secondary));
    }

    fn find(&self, secondary: SecondaryValue) -> SecondaryIterator {
        if let SecondaryValue::IdxF128(mut value) = secondary {
            let mut primary = 0;
            let ret = db_idx_long_double_find_secondary(self.code, self.scope, self.table, &mut value, &mut primary);
            return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
        }
        check(false, "IdxF128DB::find_secondary: bad secondary type");
        return SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index };
    }

    fn lowerbound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        if let SecondaryValue::IdxF128(mut value) = secondary {
            let mut primary = 0;
            let _secondary: SecondaryValue = secondary;
            let ret = db_idx_long_double_lowerbound(self.code, self.scope, self.table, &mut value, &mut primary);
            return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, _secondary);
        }
        check(false, "IdxF128DB::lowerbound: bad secondary type");
        return (SecondaryIterator{ i: -1, primary: 0, db_index: self.db_index }, SecondaryValue::IdxF128(Float128::default()));
    }

    fn upperbound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        if let SecondaryValue::IdxF128(mut value) = secondary {
            let mut primary = 0;
            let _secondary = secondary;
            let ret = db_idx_long_double_upperbound(self.code, self.scope, self.table, &mut value, &mut primary);
            return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, _secondary);    
        }
        check(false, "IdxF128DB::upperbound: bad secondary type");
        return (SecondaryIterator{ i: -1, primary: 0, db_index: 0 }, SecondaryValue::None);   
    }

    fn end(&self) -> SecondaryIterator {
        let ret = db_idx_long_double_end(self.code, self.scope, self.table);
        return SecondaryIterator{ i: ret, primary: 0, db_index: self.db_index };
    }
}
