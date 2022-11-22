use std::{
    any::Any,
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, RwLock},
};

pub type LInfo = Arc<RwLock<LocationInfo>>;
pub type EInfo = Arc<RwLock<Element>>;
pub type MInfo = Arc<RwLock<ModuleInfo>>;

pub type LRow = Arc<RwLock<Location>>;
pub type ERow = Arc<RwLock<RowElement>>;
pub type MRow = Arc<RwLock<Module>>;

use crate::{
    enums::{AdvanceEnum, CustomEnum},
    prelude::{Element, FileOrData, Location, LocationInfo, Module, ModuleInfo, RowElement},
};

#[derive(Debug, Clone)]
pub enum Type {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),

    F32(f32),
    F64(f64),

    BOOL(bool),

    String(String),
    Path(PathBuf),
    HashMapSS(HashMap<String, String>),
    HashMapS(HashMap<String, Type>),
    HashMap(HashMap<Type, Type>),
    FileOrData(FileOrData),
    Any(Arc<RwLock<Box<dyn Any>>>),
    // i hate my life
    // i want to be able to use filds
    CustomEnum(CustomEnum),
    AdvancedEnum(AdvanceEnum),
    // Fields(Box<dyn Fields>),
    Vec(Vec<Type>),
    Bytes(Vec<u8>),

    Some(Box<Type>),
    None,
}
