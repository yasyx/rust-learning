use std::{path::Path, str, convert::TryInto};
use sled::{Db, IVec};
use crate::{KvError, Kvpair, Storage, StorageIter, Value};

#[derive(Debug)]
pub struct SledDB(Db);

impl SledDB {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self(sled::open(path).unwrap())
    }
    fn get_full_key(table: &str, key: &str) -> String {
        format!("{}:{}", table, key)
    }

    fn get_table_prefix(table: &str) -> String {
        format!("{}", table)
    }
}

impl Storage for SledDB {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let name = SledDB::get_full_key(table, key);
        let result = self.0.get(name.as_bytes())?.map(|v| v.as_ref().try_into());
        flip(result)
    }

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
        let name = SledDB::get_full_key(table, &key);
        let data: Vec<u8> = value.try_into()?;
        let result = self.0.insert(name, data)?.map(|v| v.as_ref().try_into());
        flip(result)
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        let name = SledDB::get_full_key(table, &key);
        Ok(self.0.contains_key(name)?)
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let name = SledDB::get_full_key(table, &key);
        let result = self.0.remove(name)?
            .map(|v| v.as_ref().try_into());
        flip(result)
    }

    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
        let prefix = SledDB::get_table_prefix(table);
        let result = self.0.scan_prefix(prefix)
            .map(|v| v.into())
            .collect();
        Ok(result)
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item=Kvpair>>, KvError> {
        let prefix = SledDB::get_table_prefix(table);
        let iter = StorageIter::new(self.0.scan_prefix(prefix));
        Ok(Box::new(iter))
    }
}


impl From<sled::Error> for KvError {
    fn from(err: sled::Error) -> Self {
        Self::Internal(err.to_string())
    }
}

impl From<Result<(IVec, IVec), sled::Error>> for Kvpair {
    fn from(v: Result<(IVec, IVec), sled::Error>) -> Self {
        match v {
            Ok((k, v)) => match v.as_ref().try_into() {
                Ok(v) => Kvpair::new(ivec_to_key(k.as_ref()), v),
                Err(_) => Kvpair::default()
            }
            _ => Kvpair::default()
        }
    }
}

fn ivec_to_key(ivec: &[u8]) -> &str {
    let x = str::from_utf8(ivec).unwrap();
    let mut iter = x.split(":");
    iter.next();
    iter.next().unwrap()
}

fn flip<T, E>(x: Option<Result<T, E>>) -> Result<Option<T>, E> {
    x.map_or(Ok(None), |v| v.map(Some))
}