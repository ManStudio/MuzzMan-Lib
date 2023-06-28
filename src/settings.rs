use std::collections::HashMap;

pub struct Settings {
    settings: HashMap<String, Setting>,
}

pub struct Setting {
    pub value: Atom,
    default: Atom,
    variants: Vec<Atom>,
    description: String,
}

pub enum Atom {
    I(i64),
    U(u64),
    S(String),
}
