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

    Bool(bool),

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

    None,
}

impl Type {
    pub fn to_tag(&self) -> TypeTag {
        use TypeTag::*;

        match self {
            Type::U8(_) => U8,
            Type::U16(_) => U16,
            Type::U32(_) => U32,
            Type::U64(_) => U64,
            Type::U128(_) => U128,
            Type::USize(_) => USize,
            Type::I8(_) => I8,
            Type::I16(_) => I16,
            Type::I32(_) => I32,
            Type::I64(_) => I64,
            Type::I128(_) => I128,
            Type::ISize(_) => ISize,
            Type::F32(_) => F32,
            Type::F64(_) => F64,
            Type::Bool(_) => Bool,
            Type::String(_) => String,
            Type::Path(_) => Path,
            Type::HashMapSS(_) => HashMapSS,
            Type::HashMapS(h) => {
                let Some(ty) = h.iter().nth(0) else{return None;};
                HashMapS(Box::new(ty.1.to_tag()))
            }
            Type::HashMap(h) => {
                let Some(ty) = h.iter().nth(0) else{return None;};
                HashMap(Box::new(ty.0.to_tag()), Box::new(ty.1.to_tag()))
            }
            Type::FileOrData(_) => FileOrData,
            Type::Any(_) => TypeTag::Any,
            Type::CustomEnum(e) => CustomEnum(e.clone()),
            Type::AdvancedEnum(e) => AdvancedEnum(e.clone()),
            Type::Vec(v) => {
                let Some(d) = v.get(0)else{return Vec(Box::new(None))};
                Vec(Box::new(d.to_tag()))
            }
            Type::Bytes(_) => Bytes,
            Type::None => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeTag {
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,

    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,

    F32,
    F64,

    Bool,

    String,
    Url,
    Path,
    HashMapSS,
    HashMapS(Box<TypeTag>),
    HashMap(Box<TypeTag>, Box<TypeTag>),
    FileOrData,

    Any,

    CustomEnum(CustomEnum),
    AdvancedEnum(AdvanceEnum),

    Vec(Box<TypeTag>),
    Bytes,

    None,
}

impl TypeTag {
    pub fn to_string(&self) -> String {
        match self {
            TypeTag::U8 => "u8".to_string(),
            TypeTag::U16 => "u16".to_string(),
            TypeTag::U32 => "u32".to_string(),
            TypeTag::U64 => "u64".to_string(),
            TypeTag::U128 => "u128".to_string(),
            TypeTag::USize => "usize".to_string(),
            TypeTag::I8 => "i8".to_string(),
            TypeTag::I16 => "i16".to_string(),
            TypeTag::I32 => "i32".to_string(),
            TypeTag::I64 => "i64".to_string(),
            TypeTag::I128 => "i128".to_string(),
            TypeTag::ISize => "isize".to_string(),
            TypeTag::F32 => "f32".to_string(),
            TypeTag::F64 => "f64".to_string(),
            TypeTag::Bool => "bool".to_string(),
            TypeTag::String => "string".to_string(),
            TypeTag::Url => "url".to_string(),
            TypeTag::Path => "path".to_string(),
            TypeTag::HashMapSS => "hashmap_string_string".to_string(),
            TypeTag::HashMapS(h) => format!("hashmap_string({})", h.to_string()),
            TypeTag::HashMap(h1, h2) => format!("hashmap({},{})", h1.to_string(), h2.to_string()),
            TypeTag::FileOrData => "file_or_data".to_string(),
            TypeTag::Any => "any".to_string(),
            TypeTag::CustomEnum(_) => "custom_enum".to_string(),
            TypeTag::AdvancedEnum(_) => "advanced_enum".to_string(),
            TypeTag::Vec(v) => format!("vec({})", v.to_string()),
            TypeTag::Bytes => "bytes".to_string(),
            TypeTag::None => "none".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TypeValidation {
    Range(usize, usize),
}
