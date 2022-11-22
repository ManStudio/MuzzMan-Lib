use crate::types::Type;

use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct Bytes(pub Vec<u8>);

#[derive(Debug, Clone, Default)]
pub struct Data {
    pub data: HashMap<String, Type>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: Type) -> Option<Type> {
        self.data.insert(key.to_owned(), value)
    }

    pub fn get(&self, key: &str) -> Option<&Type> {
        self.data.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Type> {
        self.data.get_mut(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<Type> {
        self.data.remove(key)
    }

    pub fn default(&mut self, key: &str, value: Type) {
        if let None = self.data.get(key) {
            self.data.insert(key.to_owned(), value);
        }
    }

    pub fn iter(&mut self) -> std::collections::hash_map::Iter<String, Type> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<String, Type> {
        self.data.iter_mut()
    }
}

#[derive(Debug, Clone)]
pub enum FileOrData {
    File(PathBuf, Option<Arc<Mutex<std::fs::File>>>),
    Bytes(Bytes),
}
