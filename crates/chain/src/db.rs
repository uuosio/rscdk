use core::convert::{
    From,
    Into,
};

use crate::structs::{ Uint128, Uint256, Float128 };
use crate::serializer::Packer;

use crate::string::{
    String,
};

use crate::vmapi::db::*;

use crate::name::{
    Name,
};

use crate::serializer::{
    Encoder,
};

use crate::asset::Asset;

use crate::{
    check,
};

use crate::print::{
    Printable
};

///
#[derive(Clone, Debug, Default)]
pub struct TableError {
    ///
    pub message: String,
}

impl PrimaryValueInterface for Name {
    fn get_primary(&self) -> u64 {
        return self.value();
    }
}

impl PrimaryValueInterface for u64 {
    fn get_primary(&self) -> u64 {
        return *self;
    }
}

impl PrimaryValueInterface for Asset {
    fn get_primary(&self) -> u64 {
        return self.symbol().code().value();
    }
}

///
pub trait PrimaryValueInterface {
    ///
    fn get_primary(&self) -> u64;
}

pub trait SecondaryValueInterface: core::any::Any {
    ///
    fn get_secondary_value(&self, i: usize) -> SecondaryValue;
    ///
    fn set_secondary_value(&mut self, i: usize, value: SecondaryValue);
}

pub trait AsAny {
    fn as_any(&self) -> &dyn core::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any;
}

///
pub trait MultiIndexValue: PrimaryValueInterface + SecondaryValueInterface + Packer + AsAny {

}

pub struct Primary<'a, T>
where T: Packer + PrimaryValueInterface + Default
{
    pub(crate) value: T,
    _marker: core::marker::PhantomData<&'a ()>,
}

impl <'a, T> Primary<'a, T>
where T: Packer + PrimaryValueInterface + Default
{
    pub fn new(value: T) -> Self {
        Self { value, _marker: core::marker::PhantomData::<>{} }
    }

    pub fn get_primary(&self) -> u64 {
        self.value.get_primary()
    }

    pub fn value(self) -> T {
        self.value
    }
}

pub struct Secondary<T>
where T: Packer + Default
{
    pub(crate) value: T,
}

impl <T> Secondary<T>
where T: Packer + Default
{
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn value(self) -> T {
        self.value
    }
}

///
pub struct Iterator<'a, T> 
where T: Packer + PrimaryValueInterface + Default
{
    ///
    pub(crate) i: i32,
    pub(crate) primary: Option<u64>,
    db: &'a TableI64<T>,
}

impl<'a, T> Iterator<'a, T> 
where T: Packer + PrimaryValueInterface + Default
{
    ///
    pub fn new(i: i32, primary: Option<u64>, db: &'a TableI64<T>) -> Self {
        Self { i, primary, db }
    }

    /// get primary key of iterator
    /// access `TableI64` to extract primary key from value if primary key is not cached
    pub fn get_primary(&self) -> Option<u64> {
        if !self.is_ok() {
            return None;
        }

        if self.primary.is_some() {
            return self.primary;
        }

        return Some(self.db.get(self).unwrap().get_primary());
    }

    pub fn set_primary(&mut self, primary: u64) {
        if !self.is_ok() {
            return;
        }
        self.primary = Some(primary);
    }

    ///
    pub fn get_value(&self) -> Option<T> {
        return self.db.get(self);
    }

    ///
    pub fn get_i(&self) -> i32 {
        return self.i;
    }

    /// return `true` if iterator is valid, else `false`
    pub fn is_ok(&self) -> bool {
        self.i >= 0
    }

    /// return `true` if it's an end iterator, else `false`
    /// use this method to check the return value of `MultiIndex.end` or `TableI64.end`
    pub fn is_end(&self) -> bool {
        return self.i < -1;
    }

    ///help function for asserting on valid iterator
    pub fn expect(self, msg: &str) -> Self {
        check(self.is_ok(), msg);            
        return self;
    }

    ///help function for asserting on invalid iterator
    pub fn expect_not_ok(self, msg: &str) -> Self {
        check(!self.is_ok(), msg);            
        return self;
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
        return self.i < -1;
    }
}

impl Default for SecondaryIterator {
    fn default() -> Self {
        SecondaryIterator{ i: -1, primary: 0, db_index: usize::MAX }
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

impl From<u64> for SecondaryValue {
    fn from(value: u64) -> Self {
        SecondaryValue::Idx64(value)
    }
}

impl From<u128> for SecondaryValue {
    fn from(value: u128) -> Self {
        SecondaryValue::Idx128(value)
    }
}

impl From<Uint256> for SecondaryValue {
    fn from(value: Uint256) -> Self {
        SecondaryValue::Idx256(value)
    }
}

impl From<f64> for SecondaryValue {
    fn from(value: f64) -> Self {
        SecondaryValue::IdxF64(value)
    }
}

impl From<Float128> for SecondaryValue {
    fn from(value: Float128) -> Self {
        SecondaryValue::IdxF128(value)
    }
}

impl From<SecondaryValue> for u64 {
    fn from(value: SecondaryValue) -> Self {
        if let SecondaryValue::Idx64(x) = value {
            x
        } else {
            check(false, "From<SecondaryValue> for u64: Invalid SecondaryValue");
            Default::default()
        }
    }
}

impl From<SecondaryValue> for u128 {
    fn from(value: SecondaryValue) -> Self {
        if let SecondaryValue::Idx128(x) = value {
            x
        } else {
            check(false, "From<SecondaryValue> for u128: Invalid SecondaryValue");
            Default::default()
        }
    }
}

impl From<SecondaryValue> for Uint256 {
    fn from(value: SecondaryValue) -> Self {
        if let SecondaryValue::Idx256(x) = value {
            x
        } else {
            check(false, "From<SecondaryValue> for Uint256: Invalid SecondaryValue");
            Default::default()
        }
    }
}

impl From<SecondaryValue> for f64 {
    fn from(value: SecondaryValue) -> Self {
        if let SecondaryValue::IdxF64(x) = value {
            x
        } else {
            check(false, "From<SecondaryValue> for f64: Invalid SecondaryValue");
            Default::default()
        }
    }
}

impl From<SecondaryValue> for Float128 {
    fn from(value: SecondaryValue) -> Self {
        if let SecondaryValue::IdxF128(x) = value {
            x
        } else {
            check(false, "From<SecondaryValue> for Float128: Invalid SecondaryValue");
            Default::default()
        }
    }
}

///
pub struct TableI64<T> 
where
    T: Packer + PrimaryValueInterface + Default,
{
    ///
    pub code: u64,
    ///
    pub scope: u64,
    ///
    pub table: u64,
    _marker: core::marker::PhantomData<T>,
}

impl<T> TableI64<T>
where
    T: Packer + PrimaryValueInterface + Default,
{
    /// Creates a new TableI64 instance
    pub fn new(code: Name, scope: Name, table: Name) -> Self {
        TableI64 {
            code: code.value(),
            scope: scope.value(),
            table: table.value(),
            _marker: core::marker::PhantomData::<T>{}
        }
    }

    ///
    pub fn store(&self, value: &T, payer: Name) -> Iterator<T> {
        let key = value.get_primary();
        let data = Encoder::pack(value);
        let it = db_store_i64(self.scope, self.table, payer.value(), key, data.as_ptr(), data.len() as u32);
        Iterator::<T> { i: it, primary: Some(key), db: self }
    }

    ///
    pub fn find(&self, key: u64) -> Iterator<T> {
        let it = db_find_i64(self.code, self.scope, self.table, key);
        if it != -1 {
            Iterator::<T> { i: it, primary: Some(key), db: self }
        } else {
            Iterator::<T> { i: it, primary: None, db: self }
        }
    }

    ///
    pub fn update(&self, iterator: &Iterator<T>, value: &T, payer: Name) {
        check(iterator.is_ok(), "TableI64::update:invalid iterator");
        check(iterator.get_primary().unwrap() == value.get_primary(), "TableI64::update: can not change primary value during update!");
        let data = Encoder::pack(value);
        db_update_i64(iterator.i, payer.value(), data.as_ptr(), data.len() as u32);
    }

    /// remove value from database by iterator
    pub fn remove(&self, iterator: &Iterator<T>) {
        db_remove_i64(iterator.i);
    }

    /// get value by iterator. use [Iterator](crate::db::Iterator)::get_value for a more convenient way.
    pub fn get(&self, iterator: &Iterator<T>) -> Option<T> {
        if !iterator.is_ok() {
            return None;
        }

        let data = db_get_i64(iterator.i);
        let mut ret = T::default();
        ret.unpack(&data); 
        Some(ret)
    }

    /// get next iterator
    pub fn next(&self, iterator: &Iterator<T>) -> Iterator<T> {
        let mut primary = 0;
        let it = db_next_i64(iterator.i, &mut primary);
        if it != -1 {
            Iterator::<T> { i: it, primary: Some(primary), db: self }
        } else {
            Iterator::<T> { i: it, primary: None, db: self }
        }
    }

    /// get previous iterator
    pub fn previous(&self, iterator: &Iterator<T>) -> Iterator<T> {
        let mut primary = 0;
        let it = db_previous_i64(iterator.i, &mut primary);
        if it != -1 {
            Iterator::<T> { i: it, primary: Some(primary), db: self }
        } else {
            Iterator::<T> { i: it, primary: None, db: self }
        }
    }

    /// return a iterator with a key >= `key`
    pub fn lower_bound(&self, key: u64) -> Iterator<T> {
        let it = db_lowerbound_i64(self.code, self.scope, self.table, key);
        Iterator::<T> { i: it, primary: None, db: self }
    }

    /// return a iterator with a key > `key`
    pub fn upper_bound(&self, key: u64) -> Iterator<T> {
        let it = db_upperbound_i64(self.code, self.scope, self.table, key);
        Iterator::<T> { i: it, primary: None, db: self }
    }

    /// Return an end iterator, Iterator.is_end() return true if it's a valid end iterator.
    /// This method is often used with `TableI64.previous` to get the last iterator.

    /// ```rust
    /// let mut it = db.end();
    /// if it.is_end() {
    ///     it = db.previous();
    ///     //...
    /// }
    /// ```
    pub fn end(&self) -> Iterator<T> {
        let it = db_end_i64(self.code, self.scope, self.table);
        Iterator::<T> { i: it, primary: None, db: self }
    }
}

///
pub struct Idx64Table {
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
pub struct Idx128Table {
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
pub struct Idx256Table {
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
pub struct IdxF64Table {
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
pub struct IdxF128Table {
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
pub trait IdxTable {
    // fn new(code: u64, scope: u64, table: u64) -> Self;
    ///
    fn get_db_index(&self) -> usize;
    ///
    fn store(&self, key: u64, secondary: SecondaryValue, payer: Name) -> SecondaryIterator;
    ///
    fn update(&self, iterator: &SecondaryIterator, secondary: SecondaryValue, payer: Name);
    ///
    fn remove(&self, iterator: &SecondaryIterator);
    ///
    fn next(&self, iterator: &SecondaryIterator) -> SecondaryIterator;
    ///
    fn previous(&self, iterator: &SecondaryIterator) -> SecondaryIterator;
    ///
    fn find_primary(&self, primary: u64) -> (SecondaryIterator, SecondaryValue);
    ///
    fn find(&self, secondary: SecondaryValue) -> SecondaryIterator;
    ///
    fn lower_bound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue);
    ///
    fn upper_bound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue);
    ///
    fn end(&self) -> SecondaryIterator;
}

///
pub struct IdxTableProxy<'a, T: From<SecondaryValue> + Into<SecondaryValue> + Printable + Default, const IDX_TYPE: usize> {
    ///
    pub db: &'a dyn IdxTable,
    _secondary_type: SecondaryType,
    _marker: core::marker::PhantomData<T>,
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

impl<'a, T: From<SecondaryValue> + Into<SecondaryValue> + Printable + Default, const IDX_TYPE: usize> IdxTableProxy<'a, T, IDX_TYPE>
{
    ///
    pub fn new(db: &'a dyn IdxTable) -> Self {
        Self {
            db,
            _secondary_type: index_to_secondary_type(IDX_TYPE),
            _marker: core::marker::PhantomData::<T>{},
        }
    }
    ///
    pub fn get_db_index(&self) -> usize {
        return self.db.get_db_index();
    }

    ///
    pub fn store(&self, key: u64, value: T, payer: Name) -> SecondaryIterator {
        let _value: SecondaryValue = value.into();
        return self.db.store(key, _value, payer);
    }

    ///
    pub fn update(&self, iterator: &SecondaryIterator, value: T, payer: Name) {
        self.db.update(iterator, value.into(), payer);
    }

    ///
    pub fn remove(&self, iterator: &SecondaryIterator) {
        self.db.remove(iterator);
    }

    ///
    pub fn next(&self, iterator: &SecondaryIterator) -> SecondaryIterator {
        return self.db.next(iterator);
    }

    ///
    pub fn previous(&self, iterator: &SecondaryIterator) -> SecondaryIterator {
        return self.db.previous(iterator);
    }

    ///
    pub fn find_primary(&self, primary: u64) -> (SecondaryIterator, T) {
        let (it, value) = self.db.find_primary(primary);
        return (it, value.into());
    }

    ///
    pub fn find(&self, secondary: T) -> SecondaryIterator {
        return self.db.find(secondary.into());
    }

    ///
    pub fn lower_bound(&self, secondary: T) -> (SecondaryIterator, T) {
        let (it, value) = self.db.lower_bound(secondary.into());
        return (it, value.into());
    }

    ///
    pub fn upper_bound(&self, secondary: T) -> (SecondaryIterator, T) {
        let _secondary = secondary.into();
        let (it, value) = self.db.upper_bound(_secondary);
        let _value = value.into();
        return (it, _value);
    }

    ///
    pub fn end(&self) -> SecondaryIterator {
        return self.db.end();
    }

}

impl Idx64Table {
    ///
    pub fn new(db_index: usize, code: Name, scope: Name, table: Name) -> Self {
        Idx64Table { db_index: db_index, code: code.value(), scope: scope.value(), table: table.value() }
    }
}

impl IdxTable for Idx64Table {
    fn get_db_index(&self) -> usize {
        return self.db_index;
    }

    fn store(&self, key: u64, secondary: SecondaryValue, payer: Name) -> SecondaryIterator {
        if let SecondaryValue::Idx64(value) = secondary {
            let ret = db_idx64_store(self.scope, self.table, payer.value(), key, &value);
            return SecondaryIterator{ i: ret, primary: key, db_index: self.db_index };    
        }
        check(false, "Idx64Table::store: bad secondary type");
        return Default::default()
    }

    fn update(&self, iterator: &SecondaryIterator, secondary: SecondaryValue, payer: Name) {
        if let SecondaryValue::Idx64(value) = secondary {
            db_idx64_update(iterator.i, payer.value(), &value);
            return;
        } else {
            check(false, "Idx64Table::update: bad secondary type");
            return;
        }
    }

    fn remove(&self, iterator: &SecondaryIterator) {
        db_idx64_remove(iterator.i);
    }

    fn next(&self, iterator: &SecondaryIterator) -> SecondaryIterator {
        let mut primary = 0;
        let ret = db_idx64_next(iterator.i, &mut primary);
        return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
    }

    fn previous(&self, iterator: &SecondaryIterator) -> SecondaryIterator {
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
        check(false, "Idx64Table::find_secondary: bad secondary type");
        return Default::default();
    }

    fn lower_bound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        if let SecondaryValue::Idx64(mut value) = secondary {
            let mut primary = 0;
            let ret = db_idx64_lowerbound(self.code, self.scope, self.table, &mut value, &mut primary);
            
            return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::Idx64(value));    
        }
        check(false, "Idx64Table::lower_bound: bad secondary type");
        return (Default::default(), SecondaryValue::Idx64(0));
    }

    fn upper_bound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        if let SecondaryValue::Idx64(mut value) = secondary {
            let mut primary = 0;
            let ret = db_idx64_upperbound(self.code, self.scope, self.table, &mut value, &mut primary);            
            return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::Idx64(value));
        }
        check(false, "Idx64Table::upper_bounddd: bad secondary type");
        return (Default::default(), SecondaryValue::Idx128(0));
    }

    fn end(&self) -> SecondaryIterator {
        let ret = db_idx64_end(self.code, self.scope, self.table);
        return SecondaryIterator{ i: ret, primary: 0, db_index: self.db_index };
    }
}

impl Idx128Table {
    ///
    pub fn new(db_index: usize, code: Name, scope: Name, table: Name) -> Self {
        Idx128Table { db_index: db_index, code: code.value(), scope: scope.value(), table: table.value() }
    }
}

impl IdxTable for Idx128Table {
    fn get_db_index(&self) -> usize {
        return self.db_index;
    }

    fn store(&self, key: u64, secondary: SecondaryValue, payer: Name) -> SecondaryIterator {
        if let SecondaryValue::Idx128(value) = secondary {
            let _secondary = Uint128{lo: (value & u64::MAX as u128) as u64, hi: (value >> 64) as u64};
            let ret = db_idx128_store(self.scope, self.table, payer.value(), key, &_secondary);
            return SecondaryIterator{ i: ret, primary: key, db_index: self.db_index };
        }
        check(false, "Idx128Table::store: bad secondary type");
        return Default::default();
    }

    fn update(&self, iterator: &SecondaryIterator, secondary: SecondaryValue, payer: Name) {
        if let SecondaryValue::Idx128(value) = secondary {
            let _secondary = Uint128{lo: (value & u64::MAX as u128) as u64, hi: (value >> 64) as u64};
            db_idx128_update(iterator.i, payer.value(), &_secondary);
        } else {
            check(false, "Idx128Table::update: bad secondary type");
        }
    }

    fn remove(&self, iterator: &SecondaryIterator) {
        db_idx128_remove(iterator.i);
    }

    fn next(&self, iterator: &SecondaryIterator) -> SecondaryIterator {
        let mut primary = 0;
        let ret = db_idx128_next(iterator.i, &mut primary);
        return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
    }

    fn previous(&self, iterator: &SecondaryIterator) -> SecondaryIterator {
        let mut primary = 0;
        let ret = db_idx128_previous(iterator.i, &mut primary);
        return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
    }

    fn find_primary(&self, primary: u64) -> (SecondaryIterator, SecondaryValue) {
        //initialize Uint128
        let mut secondary = Uint128{lo:0, hi: 0};
        let ret = db_idx128_find_primary(self.code, self.scope, self.table, &mut secondary, primary);
        let _secondary = ((secondary.hi as u128) << 64) + secondary.lo as u128;
        return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::Idx128(_secondary));
    }

    fn find(&self, secondary: SecondaryValue) -> SecondaryIterator {
        if let SecondaryValue::Idx128(value) = secondary {
            let mut primary = 0;
            let mut _secondary = Uint128{lo: (value & u64::MAX as u128) as u64, hi: (value >> 64) as u64};
            let ret = db_idx128_find_secondary(self.code, self.scope, self.table, &mut _secondary, &mut primary);
            return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
        }
        check(false, "Idx128Table::find_secondary: bad secondary type");
        return Default::default();
    }

    fn lower_bound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        if let SecondaryValue::Idx128(mut value) = secondary {
            let mut primary = 0;
            // let _secondary: SecondaryValue = secondary;
            let mut _secondary = Uint128{lo: (value & u64::MAX as u128) as u64, hi: (value >> 64) as u64};
            let ret = db_idx128_lowerbound(self.code, self.scope, self.table, &mut _secondary, &mut primary);
            value = ((_secondary.hi as u128) << 64) + _secondary.lo as u128;
            return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::Idx128(value));
        }
        check(false, "Idx128Table::lower_bound: bad secondary type");
        return (Default::default(), SecondaryValue::Idx128(0));
    }

    fn upper_bound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        match secondary {
            SecondaryValue::Idx128(mut value) => {
                let mut primary = 0;
                // let _secondary = secondary;
                let mut _secondary = Uint128{lo: (value & u64::MAX as u128) as u64, hi: (value >> 64) as u64};
                let ret = db_idx128_upperbound(self.code, self.scope, self.table, &mut _secondary, &mut primary);
                value = ((_secondary.hi as u128) << 64) + _secondary.lo as u128;
                return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::Idx128(value));
            },
            _ => {
                check(false, "Idx128Table::upper_bound: bad secondary type");
                return (Default::default(), SecondaryValue::Idx128(0));
            }
        }
    }

    fn end(&self) -> SecondaryIterator {
        let ret = db_idx128_end(self.code, self.scope, self.table);
        return SecondaryIterator{ i: ret, primary: 0, db_index: self.db_index };
    }
}

impl Idx256Table {
    ///
    pub fn new(db_index: usize, code: Name, scope: Name, table: Name) -> Self {
        Idx256Table { db_index: db_index, code: code.value(), scope: scope.value(), table: table.value() }
    }
}

impl IdxTable for Idx256Table {
    fn get_db_index(&self) -> usize {
        return self.db_index;
    }

    fn store(&self, key: u64, secondary: SecondaryValue, payer: Name) -> SecondaryIterator {
        if let SecondaryValue::Idx256(value) = secondary {
            let _value = value.swap();
            let ret = db_idx256_store(self.scope, self.table, payer.value(), key, _value.data.as_ptr() as *const Uint128, 2);
            return SecondaryIterator{ i: ret, primary: key, db_index: self.db_index };
        }
        check(false, "Idx256Table::store: bad secondary type");
        return Default::default();
    }

    fn update(&self, iterator: &SecondaryIterator, secondary: SecondaryValue, payer: Name) {
        if let SecondaryValue::Idx256(value) = secondary {
            let _value = value.swap();
            db_idx256_update(iterator.i, payer.value(), _value.data.as_ptr() as *mut Uint128, 2);
        } else {
            check(false, "Idx256Table::update: bad secondary type");
        }
    }

    fn remove(&self, iterator: &SecondaryIterator) {
        db_idx256_remove(iterator.i);
    }

    fn next(&self, iterator: &SecondaryIterator) -> SecondaryIterator {
        let mut primary = 0;
        let ret = db_idx256_next(iterator.i, &mut primary);
        return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
    }

    fn previous(&self, iterator: &SecondaryIterator) -> SecondaryIterator {
        let mut primary = 0;
        let ret = db_idx256_previous(iterator.i, &mut primary);
        return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
    }

    fn find_primary(&self, primary: u64) -> (SecondaryIterator, SecondaryValue) {
        //initialize Uint128
        let mut secondary = Uint256{data: [0; 2]};
        let ret = db_idx256_find_primary(self.code, self.scope, self.table, secondary.data.as_mut_ptr() as *mut Uint128, 2, primary);
        return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::Idx256(secondary.swap()));
    }

    fn find(&self, secondary: SecondaryValue) -> SecondaryIterator {
        if let SecondaryValue::Idx256(value) = secondary {
            let mut primary = 0;
            let _value = value.swap();
            let ret = db_idx256_find_secondary(self.code, self.scope, self.table, _value.data.as_ptr() as *const Uint128, 2, &mut primary);
            return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
        }
        check(false, "Idx256Table::find_secondary: bad secondary type");
        return Default::default();
    }

    fn lower_bound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        if let SecondaryValue::Idx256(value) = secondary {
            let mut primary = 0;
            let mut _value = value.swap();
            let ret = db_idx256_lowerbound(self.code, self.scope, self.table, _value.data.as_mut_ptr() as *mut u8 as *mut Uint128, 2, &mut primary);
            if ret >= 0 {
                return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::Idx256(_value.swap()));
            } else {
                return (SecondaryIterator{ i: ret, primary: 0, db_index: self.db_index }, SecondaryValue::Idx256(Default::default()));
            }
        }
        check(false, "Idx256Table::lower_bound: bad secondary type");
        return (Default::default(), SecondaryValue::Idx128(0));
    }

    fn upper_bound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        match secondary {
            SecondaryValue::Idx256(value) => {
                let mut primary = 0;
                let mut _value = value.swap();
                let ret = db_idx256_upperbound(self.code, self.scope, self.table, _value.data.as_mut_ptr() as *mut u8 as *mut Uint128, 2, &mut primary);
                if ret >= 0 {
                    return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::Idx256(_value.swap()));
                } else {
                    return (SecondaryIterator{ i: ret, primary: 0, db_index: self.db_index }, SecondaryValue::Idx256(Default::default()));
                }
            },
            _ => {
                check(false, "Idx256Table::upper_boundd: bad secondary type");
                return (Default::default(), SecondaryValue::Idx128(0));
            }
        }
    }

    fn end(&self) -> SecondaryIterator {
        let ret = db_idx256_end(self.code, self.scope, self.table);
        return SecondaryIterator{ i: ret, primary: 0, db_index: self.db_index };
    }
}


impl IdxF64Table {
    ///
    pub fn new(db_index: usize, code: Name, scope: Name, table: Name) -> Self {
        IdxF64Table { db_index: db_index, code: code.value(), scope: scope.value(), table: table.value() }
    }
}

impl IdxTable for IdxF64Table {
    fn get_db_index(&self) -> usize {
        return self.db_index;
    }

    fn store(&self, key: u64, secondary: SecondaryValue, payer: Name) -> SecondaryIterator {
        if let SecondaryValue::IdxF64(value) = secondary {
            let ret = db_idx_double_store(self.scope, self.table, payer.value(), key, &value);
            return SecondaryIterator{ i: ret, primary: key, db_index: self.db_index };
        }
        check(false, "IdxF64Table::store: bad secondary type");
        return Default::default();
    }

    fn update(&self, iterator: &SecondaryIterator, secondary: SecondaryValue, payer: Name) {
        if let SecondaryValue::IdxF64(value) = secondary {
            db_idx_double_update(iterator.i, payer.value(), &value);
        } else {
            check(false, "IdxF64Table::update: bad secondary type")
        }
    }

    fn remove(&self, iterator: &SecondaryIterator) {
        db_idx_double_remove(iterator.i);
    }

    fn next(&self, iterator: &SecondaryIterator) -> SecondaryIterator {
        let mut primary = 0;
        let ret = db_idx_double_next(iterator.i, &mut primary);
        return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
    }

    fn previous(&self, iterator: &SecondaryIterator) -> SecondaryIterator {
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
        check(false, "IdxF64Table::find_secondary: bad secondary type");
        return Default::default();
    }

    fn lower_bound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        if let SecondaryValue::IdxF64(mut value) = secondary {
            let mut primary = 0;
            let ret = db_idx_double_lowerbound(self.code, self.scope, self.table, &mut value, &mut primary);
            return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::IdxF64(value));
        }
        check(false, "IdxF64Table::lower_bound: bad secondary type");
        return (Default::default(), SecondaryValue::IdxF64(0.0));
    }

    fn upper_bound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        match secondary {
            SecondaryValue::IdxF64(mut value) => {
                let mut primary = 0;
                let ret = db_idx_double_upperbound(self.code, self.scope, self.table, &mut value, &mut primary);
                return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::IdxF64(value));    
            },
            _ => {
                check(false, "IdxF64Table::upper_boundddd: bad secondary type");
                return (Default::default(), SecondaryValue::IdxF64(0.0));
            }
        }
    }

    fn end(&self) -> SecondaryIterator {
        let ret = db_idx_double_end(self.code, self.scope, self.table);
        return SecondaryIterator{ i: ret, primary: 0, db_index: self.db_index };
    }
}


impl IdxF128Table {
    ///
    pub fn new(db_index: usize, code: Name, scope: Name, table: Name) -> Self {
        IdxF128Table { db_index: db_index, code: code.value(), scope: scope.value(), table: table.value() }
    }
}

impl IdxTable for IdxF128Table {
    fn get_db_index(&self) -> usize {
        return self.db_index;
    }

    fn store(&self, key: u64, secondary: SecondaryValue, payer: Name) -> SecondaryIterator {
        if let SecondaryValue::IdxF128(value) = secondary {
            let ret = db_idx_long_double_store(self.scope, self.table, payer.value(), key, &value);
            return SecondaryIterator{ i: ret, primary: key, db_index: self.db_index };
        }
        check(false, "IdxF128Table::store: bad secondary type");
        return Default::default();
    }

    fn update(&self, iterator: &SecondaryIterator, secondary: SecondaryValue, payer: Name) {
        if let SecondaryValue::IdxF128(value) = secondary {
            db_idx_long_double_update(iterator.i, payer.value(), &value);
        } else {
            check(false, "IdxF128Table::update: bad secondary type")
        }
    }

    fn remove(&self, iterator: &SecondaryIterator) {
        db_idx_long_double_remove(iterator.i);
    }

    fn next(&self, iterator: &SecondaryIterator) -> SecondaryIterator {
        let mut primary = 0;
        let ret = db_idx_long_double_next(iterator.i, &mut primary);
        return SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index };
    }

    fn previous(&self, iterator: &SecondaryIterator) -> SecondaryIterator {
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
        check(false, "IdxF128Table::find_secondary: bad secondary type");
        return Default::default();
    }

    fn lower_bound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        if let SecondaryValue::IdxF128(mut value) = secondary {
            let mut primary = 0;
            let ret = db_idx_long_double_lowerbound(self.code, self.scope, self.table, &mut value, &mut primary);
            return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::IdxF128(value));
        }
        check(false, "IdxF128Table::lower_bound: bad secondary type");
        return (Default::default(), SecondaryValue::IdxF128(Float128::default()));
    }

    fn upper_bound(&self, secondary: SecondaryValue) -> (SecondaryIterator, SecondaryValue) {
        if let SecondaryValue::IdxF128(mut value) = secondary {
            let mut primary = 0;
            let ret = db_idx_long_double_upperbound(self.code, self.scope, self.table, &mut value, &mut primary);
            return (SecondaryIterator{ i: ret, primary: primary, db_index: self.db_index }, SecondaryValue::IdxF128(value));    
        }
        check(false, "IdxF128Table::upper_bound: bad secondary type");
        return (Default::default(), SecondaryValue::None);   
    }

    fn end(&self) -> SecondaryIterator {
        let ret = db_idx_long_double_end(self.code, self.scope, self.table);
        return SecondaryIterator{ i: ret, primary: 0, db_index: self.db_index };
    }
}
