#![allow(dead_code)]

use std::{collections::HashMap, hash::Hash};

use bytes_kman::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, bytes_kman::Bytes)]
pub struct CustomEnum {
    data: Vec<String>,
    pub active: Option<usize>,
    locked: bool,
}

impl PartialEq for CustomEnum {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Hash for CustomEnum {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
        self.active.hash(state);
        self.locked.hash(state);
    }
}

impl Clone for CustomEnum {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            active: self.active,
            locked: false,
        }
    }
}

impl CustomEnum {
    pub fn add(&mut self, field: &str) {
        if !self.has(field) && !self.locked {
            self.data.push(field.to_owned());
        }
    }

    pub fn has(&self, field: &str) -> bool {
        for _field in self.data.iter() {
            if _field.trim() == field.trim() {
                return true;
            }
        }

        false
    }

    pub fn get_fields(&self) -> Vec<String> {
        self.data.clone()
    }

    pub fn get_active(&self) -> Option<String> {
        if let Some(active) = self.active {
            return Some(self.data.get(active).unwrap().clone());
        }
        None
    }

    pub fn set_active(&mut self, active: Option<usize>) -> bool {
        if let Some(active) = active {
            if self.data.len() > active {
                self.active = Some(active);
                return true;
            }
            return false;
        }

        self.active = active;
        true
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }
}

#[cfg(test)]
mod test_custom_enum {
    use super::CustomEnum;

    #[test]
    fn add_field() {
        let mut custom_enum = CustomEnum::default();
        custom_enum.add("TestingField");

        assert_eq!(
            custom_enum,
            CustomEnum {
                data: vec!["TestingField".to_owned()],
                locked: false,
                active: None
            }
        );
    }

    #[test]
    fn set_active_field() {
        let mut custom_enum = CustomEnum::default();
        custom_enum.add("TestingField");
        custom_enum.add("FieldTesting");

        custom_enum.set_active(Some(1)); // FieldTesting

        assert_eq!(
            custom_enum,
            CustomEnum {
                data: vec!["TestingField".to_owned(), "FieldTesting".to_owned()],
                locked: false,
                active: Some(1)
            }
        );
    }
}

#[derive(Default, Debug, Serialize, Deserialize, bytes_kman::Bytes)]
pub struct AdvanceEnum {
    pub data: HashMap<String, bool>,
}

impl PartialEq for AdvanceEnum {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Hash for AdvanceEnum {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let _ = self
            .data
            .iter()
            .map(|(k, v)| (k.hash(state), v.hash(state)));
    }
}

impl Clone for AdvanceEnum {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl AdvanceEnum {
    fn is_active(&self, key: &str) -> bool {
        if let Some(has) = self.data.get(key) {
            *has
        } else {
            false
        }
    }

    fn set(&mut self, key: impl Into<String>, value: bool) -> Option<bool> {
        self.data.insert(key.into(), value)
    }
}
