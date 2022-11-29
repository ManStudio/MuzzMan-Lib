use crate::prelude::*;

pub struct Action {
    pub name: String,
    pub owner: MInfo,
    pub input: Vec<(String, Value)>,
    pub callback: fn(MInfo, Vec<Type>),
}
