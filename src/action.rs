use crate::prelude::*;

pub struct Action {
    pub name: String,
    pub owner: MRef,
    pub input: Vec<(String, Value)>,
    pub callback: fn(MRef, Vec<Type>),
}
