use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use rmp_serde::Serializer;
use super::error::Error;

/// A KVStore is a persistent key-value store.
/// use messagepack as serialization format.
pub struct KVStore {
    data: BTreeMap<String, Vec<u8>>,
}

impl Default for KVStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KVStore {
    /// Creates a new KVStore
    pub fn new() -> KVStore {
        KVStore { data: BTreeMap::new() }
    }

    /// Deletes a value from the store
    pub fn delete(&mut self, key: &str) -> Result<bool, Error> {
        Ok(self.data.remove(key).is_some())
    }

    /// Gets a value from the store
    pub fn get<'a, V: Deserialize<'a>>(&mut self, key: &str) -> Result<Option<V>, Error> {
        match self.data.get(key) {
            Some(buf) => {
                let mut de = rmp_serde::Deserializer::new(&buf[..]);
                let value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    /// Puts a value into the store
    pub fn set<V: Serialize>(&mut self, key: &str, value: V) -> Result<String, Error> {
        let mut buf = Vec::new();

        value.serialize(&mut Serializer::new(&mut buf)).expect("Failed to serialize");
        self.data.insert(key.to_string(), buf);
        Ok(key.to_string())
    }

}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_kvstore() {
        let mut store = KVStore::new();
        store.set("key1", "value1").unwrap();
        store.set("key2", "value2").unwrap();
        let value: Option<String> = store.get("key1").unwrap();
        assert_eq!(value, Some("value1".to_string()));
        assert!(store.delete("key1").unwrap());
        let value: Option<String> = store.get("key1").unwrap();
        assert_eq!(value, None);
    }

    #[test]
    fn test_delete() {
        let mut s = KVStore::new();

        s.set("a", "1").unwrap();
        assert_eq!("1", s.get::<String>("a").unwrap().unwrap());

        assert!(s.delete("a").unwrap());
        assert_eq!(None, s.get::<String>("a").unwrap());

        assert!(!s.delete("b").unwrap());
    }

    #[test]
    fn test_get() {
        let mut s = KVStore::new();
        s.set("a", "1").unwrap();
        assert_eq!("1", s.get::<String>("a").unwrap().unwrap());
        assert_eq!(None, s.get::<String>("b").unwrap());
    }

    #[test]
    fn test_set() {
        let mut s = KVStore::new();
        s.set("a", "1").unwrap();
        assert_eq!("1", s.get::<String>("a").unwrap().unwrap());
        s.set("a", "2").unwrap();
        assert_eq!("2", s.get::<String>("a").unwrap().unwrap());
        s.set("a", 3).unwrap();
        assert_eq!(3, s.get::<u64>("a").unwrap().unwrap())
    }

}
