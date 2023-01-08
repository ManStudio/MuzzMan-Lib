#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

impl Clone for CustomEnum {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            active: self.active.clone(),
            locked: false,
        }
    }
}

impl CustomEnum {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            active: None,
            locked: false,
        }
    }

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
        return true;
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
        let mut custom_enum = CustomEnum::new();
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
        let mut custom_enum = CustomEnum::new();
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

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvanceEnum {
    data: Vec<(String, bool)>,
    locked: bool,
}

impl PartialEq for AdvanceEnum {
    fn eq(&self, other: &Self) -> bool {
        let mut ok = 0;
        if self.data.len() == other.data.len() {
            for i in 0..self.data.len() {
                if self.data[i].0 == other.data[i].0 {
                    ok += 1;
                }
            }
        }

        ok == self.data.len()
    }
}

impl Clone for AdvanceEnum {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            locked: false,
        }
    }
}

impl AdvanceEnum {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            locked: false,
        }
    }
}
