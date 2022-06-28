use crate::db::{
    Idx64DB,
    Idx128DB,
    Idx256DB,
    // IdxF128DB,
    SecondaryType,
    SecondaryValue,
    SecondaryIterator,
    IndexDB,
    MultiIndexValue,
};

use crate::vmapi::db::*;
use crate::name::{
    Name,
};

use crate::{
    vec,
};

use crate::{
    vec::Vec,
};

use crate::{
    check,
};

use crate::boxed::Box;


///
pub struct Iterator<'a> {
    ///
    pub i: i32,
    primary: Option<u64>,
    db: &'a DBI64,
}

impl<'a> Iterator<'a> {
    ///
    pub fn new(i: i32, primary: Option<u64>, db: &'a DBI64) -> Self {
        Self { i, primary, db }
    }

    ///
    pub fn get_primary(&self) -> Option<u64> {
        if !self.is_ok() {
            return None;
        }

        if self.primary.is_some() {
            return self.primary;
        }
        
        let value = self.db.get(self).unwrap();
        return Some(value.get_primary());
    }

    ///
    pub fn set_primary(&mut self, primary: u64) {
        if !self.is_ok() {
            return;
        }
        self.primary = Some(primary);
    }

    ///
    pub fn is_ok(&self) -> bool {
        self.i >= 0
    }

    ///
    pub fn is_end(&self) -> bool {
        return self.i < -1;
    }

    ///
    pub fn expect(self, msg: &str) -> Self {
        check(self.is_ok(), msg);            
        return self;
    }

    ///
    pub fn expect_not_ok(self, msg: &str) -> Self {
        check(!self.is_ok(), msg);            
        return self;
    }
}


///
pub struct DBI64 {
    ///
    pub code: u64,
    ///
    pub scope: u64,
    ///
    pub table: u64,
    unpacker: fn(&[u8]) -> Box<dyn MultiIndexValue>,
}

impl DBI64 {
    ///
    pub fn new(code: Name, scope: Name, table: Name, unpacker: fn(&[u8]) -> Box<dyn MultiIndexValue>) -> Self {
        DBI64 {
            code: code.value(),
            scope: scope.value(),
            table: table.value(),
            unpacker,
        }
    }

    ///
    pub fn store(&self, id: u64,  data: &[u8], payer: Name) -> Iterator {
        let it = db_store_i64(self.scope, self.table, payer.value(), id, data.as_ptr(), data.len() as u32);
        Iterator { i: it, primary: Some(id), db: self }
    }

    ///
    pub fn update(&self, iterator: &Iterator, data: &[u8], payer: Name) {
        db_update_i64(iterator.i, payer.value(), data.as_ptr(), data.len() as u32);
    }

    ///
    pub fn remove(&self, iterator: &Iterator) {
        db_remove_i64(iterator.i);
    }

    ///
    pub fn get(&self, iterator: &Iterator) -> Option<Box<dyn MultiIndexValue>> {
        if !iterator.is_ok() {
            return None;
        }

        let size = db_get_i64(iterator.i, 0 as *const u8, 0);
        let mut data: Vec<u8> = vec![0; size as usize];
        // let mut data: Vec<u8> = Vec::with_capacity(size as usize);
        // data.resize_with(size as usize, Default::default);
        let ptr = data.as_mut_ptr();
        db_get_i64(iterator.i, ptr, size as u32);
        return Some((self.unpacker)(&data));
    }

    ///
    pub fn next(&self, iterator: &Iterator) -> Iterator {
        let mut primary = 0;
        let it = db_next_i64(iterator.i, &mut primary);
        if it >= 0 {
            Iterator { i: it, primary: Some(primary), db: self }
        } else {
            Iterator { i: it, primary: None, db: self }
        }
    }

    ///
    pub fn previous(&self, iterator: &Iterator) -> Iterator {
        let mut primary = 0;
        let it = db_previous_i64(iterator.i, &mut primary);
        if it >= 0 {
            Iterator { i: it, primary: Some(primary), db: self }
        } else {
            Iterator { i: it, primary: None, db: self }
        }
    }

    ///
    pub fn find(&self, primary_key: u64) -> Iterator {
        let it = db_find_i64(self.code, self.scope, self.table, primary_key);
        Iterator { i: it, primary: Some(primary_key), db: self }
    }

    ///
    pub fn lowerbound(&self, id: u64) -> Iterator {
        let it = db_lowerbound_i64(self.code, self.scope, self.table, id);
        Iterator { i: it, primary: None, db: self }
    }

    ///
    pub fn upperbound(&self, id: u64) -> Iterator {
        let it = db_upperbound_i64(self.code, self.scope, self.table, id);
        Iterator { i: it, primary: None, db: self }
    }

    ///
    pub fn end(&self) -> Iterator {
        let it = db_end_i64(self.code, self.scope, self.table);
        Iterator { i: it, primary: None, db: self }
    }
}

///
pub struct MultiIndex {
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

impl MultiIndex {
    ///
    pub fn new(code: Name, scope: Name, table: Name, indexes: &[SecondaryType], unpacker: fn(&[u8]) -> Box<dyn MultiIndexValue>) -> Self {
        let mut idxdbs: Vec<Box<dyn IndexDB>> = Vec::new();
        let mut i: usize = 0;
        let idx_table = table.value() & 0xfffffffffffffff0;
        for idx in indexes {
            match idx {
                SecondaryType::Idx64 => idxdbs.push(
                    Box::new(Idx64DB::new(i, code, scope, Name::from_u64(idx_table + i as u64)))
                ),
                SecondaryType::Idx128 => idxdbs.push(
                    Box::new(Idx128DB::new(i, code, scope, Name::from_u64(idx_table + i as u64)))
                ),
                SecondaryType::Idx256 => idxdbs.push(
                    Box::new(Idx256DB::new(i, code, scope, Name::from_u64(idx_table + i as u64)))
                ),
                _ => panic!("unsupported secondary index type"),
            }
            i += 1;
        }
        MultiIndex {
            code,
            scope,
            table,
            db: DBI64::new(code, scope, table, unpacker),
            idxdbs,
            unpacker: unpacker,
        }
    }

    ///
    pub fn store(&self, value: &dyn MultiIndexValue, payer: Name) -> Iterator {
        let primary = value.get_primary();
        for i in 0..self.idxdbs.len() {
            let v2 = value.get_secondary_value(i);
            self.idxdbs[i].store(primary, v2, payer);
        }
        let it = self.db.store(primary, &value.pack(), payer);
        return it;
    }

    ///
    pub fn update(&self, iterator: &Iterator, value: &dyn MultiIndexValue, payer: Name) {
        check(iterator.is_ok(), "update: invalid iterator");
        let primary = iterator.get_primary().unwrap();
        for i in 0..self.idxdbs.len() {
            let v2 = value.get_secondary_value(i);
            let (it_secondary, secondary_value) = self.idxdbs[i].find_primary(primary);
            if secondary_value == v2 {
                continue;
            }
            self.idxdbs[i].update(it_secondary, v2, payer);
        }
        self.db.update(iterator, &value.pack(), payer);
    }

    ///
    pub fn remove(&self, iterator: &Iterator) {
        if !iterator.is_ok() {
            return;
        }
        let primary = iterator.get_primary().unwrap();

        for i in 0..self.idxdbs.len() {
            let (it_secondary, _) = self.idxdbs[i].find_primary(primary);
            self.idxdbs[i].remove(it_secondary);
        }
        self.db.remove(iterator);
    }

    ///
    pub fn get(&self, iterator: &Iterator) -> Option<Box<dyn MultiIndexValue>> {
        if !iterator.is_ok() {
            return None;
        }

        return self.db.get(iterator);
    }

    ///
    pub fn get_by_primary(&self, primary: u64) -> Option<Box<dyn MultiIndexValue>> {
        let it = self.db.find(primary);
        return self.get(&it);
    }

    ///
    pub fn next(&self, iterator: &Iterator) -> Iterator {
        return self.db.next(iterator);
    }

    ///
    pub fn previous(&self, iterator: &Iterator) -> Iterator {
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
        check(it.is_ok(), "idx_update: invalid iterator");

        let it_primary = self.find(it.primary);
        let mut db_value = self.get(&it_primary).unwrap();
        let idx_db = self.idxdbs[it.db_index].as_ref();
        db_value.set_secondary_value(idx_db.get_db_index(), value);
        self.update(&it_primary, db_value.as_ref(), payer);
        idx_db.update(it, value, payer);    
    }
}
