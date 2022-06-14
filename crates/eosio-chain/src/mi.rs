use crate::db::*;
use crate::name::{
    Name,
};

use crate::{
    check,
};

use crate::{
    vec::Vec,
};

use crate::boxed::Box;
///
pub struct MultiIndex<T> {
    ///
    pub code: Name,
    ///
    pub scope: Name,
    ///
    pub table: Name,
    ///
    pub db: DBI64,
    ///
    pub idxdbs: Vec<Box<dyn IndexDB>>,
    ///
    pub unpacker: fn(&[u8]) -> Box<T>,
}

impl<T: MultiIndexValue> MultiIndex<T> {
    ///
    pub fn new<'a>(code: Name, scope: Name, table: Name, indexes: &[SecondaryType], unpacker: fn(&[u8]) -> Box<T>) -> Self {
        let mut idxdbs: Vec<Box<dyn IndexDB>> = Vec::new();
        let mut i: usize = 0;
        let idx_table = table.n & 0xfffffffffffffff0;
        for idx in indexes {
            match idx {
                SecondaryType::Idx64 => idxdbs.push(Box::new(Idx64DB::new(i, code.n, scope.n, idx_table + i as u64))),
                SecondaryType::Idx128 => idxdbs.push(Box::new(Idx128DB::new(i, code.n, scope.n, idx_table + i as u64))),
                SecondaryType::Idx256 => idxdbs.push(Box::new(Idx256DB::new(i, code.n, scope.n, idx_table + i as u64))),
                SecondaryType::IdxF64 => idxdbs.push(Box::new(IdxF64DB::new(i, code.n, scope.n, idx_table + i as u64))),
                SecondaryType::IdxF128 => idxdbs.push(Box::new(IdxF128DB::new(i, code.n, scope.n, idx_table + i as u64))),
                _ => check(false, "unsupported secondary index type"),
            }
            i += 1;
        }
        MultiIndex {
            code,
            scope,
            table,
            db: DBI64::new(code.n, scope.n, table.n),
            idxdbs,
            unpacker: unpacker,
        }
    }

    ///
    pub fn store<'a>(&self, value: &'a T, payer: Name) -> Iterator {
        let primary = value.get_primary();
        for i in 0..self.idxdbs.len() {
            let v2 = value.get_secondary_value(i);
            self.idxdbs[i].store(payer.n, primary, v2);
        }
        let it = self.db.store(payer.n, primary, &value.pack());
        return it;
    }

    ///
    pub fn update<'a>(&self, iterator: Iterator, value: &'a T, payer: Name) {
        let raw = self.db.get(iterator);
        // let old_value = MultiIndexValue::unpack(&raw);
        let old_value = (self.unpacker)(&raw);
        let primary = old_value.get_primary();
        for i in 0..self.idxdbs.len() {
            let v2 = value.get_secondary_value(i);
            let (it_secondary, secondary_value) = self.idxdbs[i].find_primary(primary);
            if secondary_value == v2 {
                continue;
            }
            self.idxdbs[i].update(it_secondary, v2, payer.n);
        }
        self.db.update(iterator, &value.pack(), payer.n);
    }

    ///
    pub fn remove(&self, iterator: Iterator) {
        let raw = self.db.get(iterator);
        let old_value = (self.unpacker)(&raw);
        let primary = old_value.get_primary();
        for i in 0..self.idxdbs.len() {
            let (it_secondary, _) = self.idxdbs[i].find_primary(primary);
            self.idxdbs[i].remove(it_secondary);
        }
        self.db.remove(iterator);
    }

    ///
    pub fn get(&self, iterator: Iterator) -> Option<Box<T>> {
        if !iterator.is_ok() {
            return None;
        }
        let raw = self.db.get(iterator);
        return Some((self.unpacker)(&raw));
    }

    ///
    pub fn get_by_primary(&self, primary: u64) -> Option<Box<T>> {
        let it = self.db.find(primary);
        return self.get(it);
    }

    ///
    pub fn next(&self, iterator: Iterator) -> (Iterator, u64) {
        return self.db.next(iterator);
    }

    ///
    pub fn previous(&self, iterator: Iterator) -> (Iterator, u64) {
        return self.db.previous(iterator);
    }

    ///
    pub fn find(&self, id: u64) -> Iterator {
        return self.db.find(id);
    }

    ///
    pub fn lowerbound(&self, id: u64) -> Iterator {
        return self.db.lowerbound(id);
    }

    ///
    pub fn upperbound(&self, id: u64) -> Iterator {
        return self.db.upperbound(id);
    }

    ///
    pub fn end(&self) -> Iterator {
        return self.db.end();
    }

    ///
    pub fn get_idx_db(&self, i: usize) -> &dyn IndexDB {
        return self.idxdbs[i].as_ref();
    }

    ///
    pub fn idx_update(&self, it: SecondaryIterator, value: SecondaryValue, payer: Name) {
        let it_primary = self.find(it.primary);
        if let Some(mut db_value) = self.get(it_primary) {
            let idx_db = self.idxdbs[it.db_index].as_ref();
            db_value.set_secondary_value(idx_db.get_db_index(), value);
            self.update(it_primary, db_value.as_ref(), payer);
            idx_db.update(it, value, payer.n);    
        } else {

        }
    }
}

///
pub struct MultiIndexNotGeneric {
    ///
    pub code: Name,
    ///
    pub scope: Name,
    ///
    pub table: Name,
    ///
    pub db: DBI64,
    ///
    pub idxdbs: Vec<Box<dyn IndexDB>>,
    ///
    pub unpacker: fn(&[u8]) -> Box<dyn MultiIndexValue>,
}

impl MultiIndexNotGeneric {
    ///
    pub fn new<'a>(code: Name, scope: Name, table: Name, indexes: &[SecondaryType], unpacker: fn(&[u8]) -> Box<dyn MultiIndexValue>) -> Self {
        let mut idxdbs: Vec<Box<dyn IndexDB>> = Vec::new();
        let mut i: usize = 0;
        let idx_table = table.n & 0xfffffffffffffff0;
        for idx in indexes {
            match idx {
                SecondaryType::Idx64 => idxdbs.push(Box::new(Idx64DB::new(i, code.n, scope.n, idx_table + i as u64))),
                SecondaryType::Idx128 => idxdbs.push(Box::new(Idx128DB::new(i, code.n, scope.n, idx_table + i as u64))),
                SecondaryType::Idx256 => idxdbs.push(Box::new(Idx256DB::new(i, code.n, scope.n, idx_table + i as u64))),
                _ => panic!("unsupported secondary index type"),
            }
            i += 1;
        }
        MultiIndexNotGeneric {
            code,
            scope,
            table,
            db: DBI64::new(code.n, scope.n, table.n),
            idxdbs,
            unpacker: unpacker,
        }
    }

    ///
    pub fn store<'a>(&self, value: &'a dyn MultiIndexValue, payer: Name) -> Iterator {
        let primary = value.get_primary();
        for i in 0..self.idxdbs.len() {
            let v2 = value.get_secondary_value(i);
            self.idxdbs[i].store(payer.n, primary, v2);
        }
        let it = self.db.store(payer.n, primary, &value.pack());
        return it;
    }

    ///
    pub fn update<'a>(&self, iterator: Iterator, value: &'a dyn MultiIndexValue, payer: Name) {
        let raw = self.db.get(iterator);
        // let old_value = MultiIndexValue::unpack(&raw);
        let old_value = (self.unpacker)(&raw);
        let primary = old_value.get_primary();
        for i in 0..self.idxdbs.len() {
            let v2 = value.get_secondary_value(i);
            let (it_secondary, secondary_value) = self.idxdbs[i].find_primary(primary);
            if secondary_value == v2 {
                continue;
            }
            self.idxdbs[i].update(it_secondary, v2, payer.n);
        }
        self.db.update(iterator, &value.pack(), payer.n);
    }

    ///
    pub fn remove(&self, iterator: Iterator) {
        let raw = self.db.get(iterator);
        let old_value = (self.unpacker)(&raw);
        let primary = old_value.get_primary();
        for i in 0..self.idxdbs.len() {
            let (it_secondary, _) = self.idxdbs[i].find_primary(primary);
            self.idxdbs[i].remove(it_secondary);
        }
        self.db.remove(iterator);
    }

    ///
    pub fn get(&self, iterator: Iterator) -> Option<Box<dyn MultiIndexValue>> {
        if !iterator.is_ok() {
            return None;
        }
        let raw = self.db.get(iterator);
        return Some((self.unpacker)(&raw));
    }

    ///
    pub fn get_by_primary(&self, primary: u64) -> Option<Box<dyn MultiIndexValue>> {
        let it = self.db.find(primary);
        return self.get(it);
    }

    ///
    pub fn next(&self, iterator: Iterator) -> (Iterator, u64) {
        return self.db.next(iterator);
    }

    ///
    pub fn previous(&self, iterator: Iterator) -> (Iterator, u64) {
        return self.db.previous(iterator);
    }

    ///
    pub fn find(&self, id: u64) -> Iterator {
        return self.db.find(id);
    }

    ///
    pub fn lowerbound(&self, id: u64) -> Iterator {
        return self.db.lowerbound(id);
    }

    ///
    pub fn upperbound(&self, id: u64) -> Iterator {
        return self.db.upperbound(id);
    }

    ///
    pub fn end(&self) -> Iterator {
        return self.db.end();
    }

    ///
    pub fn get_idx_db(&self, i: usize) -> &dyn IndexDB {
        return self.idxdbs[i].as_ref();
    }

    ///
    pub fn idx_update(&self, it: SecondaryIterator, value: SecondaryValue, payer: Name) {
        let it_primary = self.find(it.primary);
        if let Some(mut db_value) = self.get(it_primary) {
            let idx_db = self.idxdbs[it.db_index].as_ref();
            db_value.set_secondary_value(idx_db.get_db_index(), value);
            self.update(it_primary, db_value.as_ref(), payer);
            idx_db.update(it, value, payer.n);    
        } else {

        }
    }
}