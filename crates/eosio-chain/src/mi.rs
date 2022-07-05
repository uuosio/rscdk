use crate::db::*;
use crate::name::{
    Name,
};

// use crate::{
//     check,
// };

use crate::{
    vec::Vec,
};

use crate::{
    check,
    serializer::Packer,
};

use crate::boxed::Box;
///
pub struct MultiIndex<T>
where T: PrimaryValueInterface + SecondaryValueInterface + Packer + Default
{
    ///
    pub code: Name,
    ///
    pub scope: Name,
    ///
    pub table: Name,
    ///
    pub db: TableI64<T>,
    ///
    pub idxdbs: Vec<Box<dyn IdxTable>>,
    _marker: core::marker::PhantomData<T>,
}

impl<T> MultiIndex<T> 
where T: PrimaryValueInterface + SecondaryValueInterface + Packer + Default
{
    ///
    pub fn new(code: Name, scope: Name, table: Name, indexes: &[SecondaryType]) -> Self {
        let mut idxdbs: Vec<Box<dyn IdxTable>> = Vec::new();
        let mut i: usize = 0;
        let idx_table = table.value() & 0xfffffffffffffff0;
        for idx in indexes {
            match idx {
                SecondaryType::Idx64 => idxdbs.push(
                    Box::new(
                        Idx64Table::new(i, code, scope, Name::from_u64(idx_table + i as u64))
                    )
                ),
                SecondaryType::Idx128 => idxdbs.push(
                    Box::new(
                        Idx128Table::new(i, code, scope, Name::from_u64(idx_table + i as u64))
                    )
                ),
                SecondaryType::Idx256 => idxdbs.push(
                    Box::new(
                        Idx256Table::new(i, code, scope, Name::from_u64(idx_table + i as u64))
                    )
                ),
                SecondaryType::IdxF64 => idxdbs.push(
                    Box::new(
                        IdxF64Table::new(i, code, scope, Name::from_u64(idx_table + i as u64))
                    )
                ),
                SecondaryType::IdxF128 => idxdbs.push(
                    Box::new(
                        IdxF128Table::new(i, code, scope, Name::from_u64(idx_table + i as u64))
                    )
                ),
                // _ => check(false, "unsupported secondary index type"),
            }
            i += 1;
        }
        MultiIndex {
            code,
            scope,
            table,
            db: TableI64::new(code, scope, table),
            idxdbs,
            _marker: core::marker::PhantomData::<T>{},
        }
    }

    ///
    pub fn store(&self, value: &T, payer: Name) -> Iterator<T> {
        let primary = value.get_primary();
        for i in 0..self.idxdbs.len() {
            let v2 = value.get_secondary_value(i);
            self.idxdbs[i].store(primary, v2, payer);
        }
        let it = self.db.store(&value, payer);
        return it;
    }

    ///
    pub fn find(&self, id: u64) -> Iterator<T> {
        return self.db.find(id);
    }

    ///
    pub fn update(&self, iterator: &Iterator<T>, value: &T, payer: Name) {
        check(iterator.is_ok(), "MultiIndex::update: invalid iterator");
        let primary = iterator.get_primary().unwrap();
        self.db.update(&iterator, value, payer);
        for i in 0..self.idxdbs.len() {
            let v2 = value.get_secondary_value(i);
            let (it_secondary, secondary_value) = self.idxdbs[i].find_primary(primary);
            if secondary_value == v2 {
                continue;
            }
            self.idxdbs[i].update(&it_secondary, v2, payer);
        }
    }

    ///
    pub fn remove(&self, iterator: &Iterator<T>) {
        check(iterator.is_ok(), "remove: invalid iterator");
        let primary = iterator.get_primary().unwrap();
        for i in 0..self.idxdbs.len() {
            let (it_secondary, _) = self.idxdbs[i].find_primary(primary);
            self.idxdbs[i].remove(&it_secondary);
        }
        self.db.remove(&iterator);
    }

    ///
    pub fn get(&self, iterator: &Iterator<T>) -> Option<T> {
        if !iterator.is_ok() {
            return None;
        }
        self.db.get(iterator)
    }

    ///
    pub fn get_by_primary(&self, primary: u64) -> Option<T> {
        let it = self.db.find(primary);
        return self.get(&it);
    }

    ///
    pub fn next(&self, iterator: &Iterator<T>) -> Iterator<T> {
        return self.db.next(iterator);
    }

    ///
    pub fn previous(&self, iterator: &Iterator<T>) -> Iterator<T> {
        return self.db.previous(iterator);
    }

    ///
    pub fn lowerbound(&self, id: u64) -> Iterator<T> {
        return self.db.lowerbound(id);
    }

    ///
    pub fn upperbound(&self, id: u64) -> Iterator<T> {
        return self.db.upperbound(id);
    }

    ///
    pub fn end(&self) -> Iterator<T> {
        return self.db.end();
    }

    ///
    pub fn get_idx_db(&self, i: usize) -> &dyn IdxTable {
        return self.idxdbs[i].as_ref();
    }

    ///
    pub fn idx_update(&self, it: &SecondaryIterator, value: SecondaryValue, payer: Name) {
        let it_primary = self.find(it.primary).expect("idx_update: invalid primary");
        if let Some(mut db_value) = it_primary.get_value() {
            let idx_db = self.idxdbs[it.db_index].as_ref();
            db_value.set_secondary_value(idx_db.get_db_index(), value);
            self.update(&it_primary, &db_value, payer);
            idx_db.update(it, value, payer);    
        } else {
        }
    }
}
