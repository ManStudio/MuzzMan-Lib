use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Settings {
    settings: HashMap<String, Setting>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Setting {
    pub value: Atom,
    default: Atom,
    variants: Vec<Atom>,
    description: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Atom {
    I(i64),
    U(u64),
    S(String),
}
