use std::{
    collections::HashMap,
    ops::{RangeFull, RangeInclusive},
};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Settings {
    settings: HashMap<String, Setting>,
}

impl Settings {
    pub fn add(&mut self, name: impl Into<String>, setting: impl Into<Setting>) {
        self.settings.insert(name.into(), setting.into());
    }

    pub fn get(&self, name: impl Into<String>) -> Option<&Setting> {
        self.settings.get(&name.into())
    }

    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        for (name, setting) in self.settings.iter() {
            if !setting.validate() {
                errors.push(name.clone())
            }
        }

        errors
    }

    pub fn set_default(&mut self, name: impl Into<String>) {
        if let Some(setting) = self.settings.get_mut(&name.into()) {
            setting.set_default();
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Setting {
    pub value: Atom,
    default: Atom,
    variants: Vec<Atom>,
    description: String,
}

impl Setting {
    pub fn new(
        default: impl Into<Atom>,
        variants: Vec<impl Into<Atom>>,
        desc: impl Into<String>,
    ) -> Self {
        let default: Atom = default.into();
        Self {
            value: default.clone(),
            default,
            variants: variants
                .into_iter()
                .map(|v| v.into())
                .collect::<Vec<Atom>>(),
            description: desc.into(),
        }
    }

    pub fn validate(&self) -> bool {
        if self.variants.is_empty() {
            return true;
        }

        for variant in self.variants.iter() {
            if *variant == self.value {
                return true;
            }
        }
        false
    }

    pub fn set_default(&mut self) {
        self.value = self.default.clone();
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Atom {
    I(i64),
    U(u64),
    F(f64),
    S(String),
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::I(v) => v.fmt(f),
            Atom::U(v) => v.fmt(f),
            Atom::F(v) => v.fmt(f),
            Atom::S(v) => v.fmt(f),
        }
    }
}

impl From<String> for Atom {
    fn from(value: String) -> Self {
        Self::S(value)
    }
}

impl From<&str> for Atom {
    fn from(value: &str) -> Self {
        Self::S(value.to_string())
    }
}

impl From<i32> for Atom {
    fn from(value: i32) -> Self {
        Self::I(value as i64)
    }
}

impl From<i64> for Atom {
    fn from(value: i64) -> Self {
        Self::I(value)
    }
}

impl From<u64> for Atom {
    fn from(value: u64) -> Self {
        Self::U(value)
    }
}

impl From<f64> for Atom {
    fn from(value: f64) -> Self {
        Self::F(value)
    }
}
