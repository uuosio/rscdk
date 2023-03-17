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
    /// Creates a new instance of the MultiIndex struct.
    pub fn new(code: Name, scope: Name, table: Name, indices: &[SecondaryType]) -> Self {
        let mut idxdbs: Vec<Box<dyn IdxTable>> = Vec::new();
        let mut i: usize = 0;
        let idx_table = table.value() & 0xfffffffffffffff0;
        for idx in indices {
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

    /// Stores a given value into the table with a specified payer.
    pub fn store(&self, value: &T, payer: Name) -> Iterator<T> {
        let primary = value.get_primary();
        for i in 0..self.idxdbs.len() {
            let v2 = value.get_secondary_value(i);
            self.idxdbs[i].store(primary, v2, payer);
        }
        let it = self.db.store(&value, payer);
        return it;
    }

    /// Searches for a record with a given primary key and returns an iterator.
    pub fn find(&self, id: u64) -> Iterator<T> {
        return self.db.find(id);
    }

    /// Updates the record pointed by the given iterator with a new value and payer.
    pub fn update(&self, iterator: &Iterator<T>, value: &T, payer: Name) {
        check(iterator.is_ok(), "MultiIndex::update: invalid iterator");
        let primary = iterator.get_primary().unwrap();
        self.db.update(&iterator,value, payer);
        for i in 0..self.idxdbs.len() {
            let v2 = value.get_secondary_value(i);
            let (it_secondary, secondary_value) = self.idxdbs[i].find_primary(primary);
            if secondary_value == v2 {
                continue;
            }
            self.idxdbs[i].update(&it_secondary, v2, payer);
        }
    }

    /// Removes the record pointed by the given iterator.
    pub fn remove(&self, iterator: &Iterator<T>) {
        check(iterator.is_ok(), "remove: invalid iterator");
        let primary = iterator.get_primary().unwrap();
        for i in 0..self.idxdbs.len() {
            let (it_secondary, _) = self.idxdbs[i].find_primary(primary);
            self.idxdbs[i].remove(&it_secondary);
        }
        self.db.remove(&iterator);
    }

    /// Retrieves the value pointed by the given iterator.
    pub fn get(&self, iterator: &Iterator<T>) -> Option<T> {
        if !iterator.is_ok() {
            return None;
        }
        self.db.get(iterator)
    }

    /// Retrieves the value by its primary key.
    pub fn get_by_primary(&self, primary: u64) -> Option<T> {
        let it = self.db.find(primary);
        return self.get(&it);
    }

    /// Returns an iterator pointing to the next record.
    pub fn next(&self, iterator: &Iterator<T>) -> Iterator<T> {
        return self.db.next(iterator);
    }

    /// Returns an iterator pointing to the previous record.
    pub fn previous(&self, iterator: &Iterator<T>) -> Iterator<T> {
        return self.db.previous(iterator);
    }

    /// Returns an iterator pointing to the first record that is not less than the given primary key.
    pub fn lower_bound(&self, id: u64) -> Iterator<T> {
        return self.db.lower_bound(id);
    }

    /// Returns an iterator pointing to the first record that is greater than the given primary key.
    pub fn upper_bound(&self, id: u64) -> Iterator<T> {
        return self.db.upper_bound(id);
    }

    /// Returns an iterator pointing to the end of the table.
    pub fn end(&self) -> Iterator<T> {
        return self.db.end();
    }

    /// Retrieves a reference to the secondary index database at the given index.
    pub fn get_idx_db(&self, i: usize) -> &dyn IdxTable {
        return self.idxdbs[i].as_ref();
    }

    /// Updates the secondary index with a given iterator, value, and payer.
    pub fn idx_update(&self, it: &SecondaryIterator, value: SecondaryValue, payer: Name) {
        let it_primary = self.find(it.primary).expect("idx_update: invalid primary");
        let mut db_value = it_primary.get_value().unwrap();
        let idx_db = self.get_idx_db(it.db_index);
        db_value.set_secondary_value(idx_db.get_db_index(), value);
        self.update(&it_primary, &db_value, payer);
        idx_db.update(it, value, payer);    
    }
}
