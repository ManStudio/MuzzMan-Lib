use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Settings {
    settings: HashMap<String, Setting>,
}

impl Settings {
    pub fn add(&mut self, name: impl Into<String>, setting: impl Into<Setting>) {
        self.settings.insert(name.into(), setting.into());
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
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Atom {
    I(i64),
    U(u64),
    S(String),
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
