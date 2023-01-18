use std::{
    any::Any,
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    path::PathBuf,
    sync::{Arc, RwLock},
};

pub type LRef = Arc<RwLock<RefLocation>>;
pub type ERef = Arc<RwLock<RefElement>>;
pub type MRef = Arc<RwLock<RefModule>>;

pub type LRow = Arc<RwLock<Location>>;
pub type ERow = Arc<RwLock<Element>>;
pub type MRow = Arc<RwLock<Module>>;

#[derive(Debug, Clone)]
pub enum ID {
    Element(ElementId),
    Location(LocationId),
}

impl PartialEq for ID {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ID::Element(e) => {
                if let ID::Element(se) = other {
                    e == se
                } else {
                    false
                }
            }
            ID::Location(l) => {
                if let ID::Location(sl) = other {
                    l == sl
                } else {
                    false
                }
            }
        }
    }
}

impl ID {
    pub fn get_ref(&self, session: &dyn TSession) -> Result<Ref, SessionError> {
        match self {
            ID::Element(e) => Ok(Ref::Element(session.get_element_ref(e)?)),
            ID::Location(l) => Ok(Ref::Location(session.get_location_ref(l)?)),
        }
    }
}

use serde::{Deserialize, Serialize};

use crate::{
    element::ElementId,
    enums::{AdvanceEnum, CustomEnum},
    prelude::{
        Element, FileOrData, Location, LocationId, Module, Ref, RefElement, RefLocation, RefModule,
        SessionError, TSession,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    // HashMap(HashMap<Type, Type>),
    FileOrData(FileOrData),
    #[serde(skip)]
    Any(Arc<RwLock<Box<dyn Any>>>),
    CustomEnum(CustomEnum),
    AdvancedEnum(AdvanceEnum),
    // Fields(Box<dyn Fields>),
    Vec(Vec<Type>),
    Bytes(Vec<u8>),

    None,
}

impl Hash for Type {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Type::U8(i) => i.hash(state),
            Type::U16(i) => i.hash(state),
            Type::U32(i) => i.hash(state),
            Type::U64(i) => i.hash(state),
            Type::U128(i) => i.hash(state),
            Type::USize(i) => i.hash(state),
            Type::I8(i) => i.hash(state),
            Type::I16(i) => i.hash(state),
            Type::I32(i) => i.hash(state),
            Type::I64(i) => i.hash(state),
            Type::I128(i) => i.hash(state),
            Type::ISize(i) => i.hash(state),
            Type::F32(f) => (*f as i32).hash(state),
            Type::F64(f) => (*f as i64).hash(state),
            Type::Bool(b) => b.hash(state),
            Type::String(s) => s.hash(state),
            Type::Path(p) => p.hash(state),
            Type::HashMapSS(h) => {
                for (k, e) in h.iter() {
                    k.hash(state);
                    e.hash(state)
                }
            }
            Type::HashMapS(h) => {
                for k in h.keys() {
                    k.hash(state)
                }
            }
            Type::FileOrData(ford) => ford.hash(state),
            Type::Any(_) => 21.hash(state),
            Type::CustomEnum(e) => e.hash(state),
            Type::AdvancedEnum(e) => e.hash(state),
            Type::Vec(v) => v.hash(state),
            Type::Bytes(b) => b.hash(state),
            Type::None => 0.hash(state),
        }
    }
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
                let Some(ty) = h.iter().next() else{return None;};
                HashMapS(Box::new(ty.1.to_tag()))
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

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::U8(v) => (v as &dyn Display).fmt(f),
            Type::U16(v) => (v as &dyn Display).fmt(f),
            Type::U32(v) => (v as &dyn Display).fmt(f),
            Type::U64(v) => (v as &dyn Display).fmt(f),
            Type::U128(v) => (v as &dyn Display).fmt(f),
            Type::USize(v) => (v as &dyn Display).fmt(f),
            Type::I8(v) => (v as &dyn Display).fmt(f),
            Type::I16(v) => (v as &dyn Display).fmt(f),
            Type::I32(v) => (v as &dyn Display).fmt(f),
            Type::I64(v) => (v as &dyn Display).fmt(f),
            Type::I128(v) => (v as &dyn Display).fmt(f),
            Type::ISize(v) => (v as &dyn Display).fmt(f),
            Type::F32(v) => (v as &dyn Display).fmt(f),
            Type::F64(v) => (v as &dyn Display).fmt(f),
            Type::Bool(v) => (v as &dyn Display).fmt(f),
            Type::String(s) => f.write_str(s),
            Type::Path(v) => f.write_str(if let Some(str) = v.to_str() {
                str
            } else {
                "Cannot parse!"
            }),
            Type::HashMapSS(v) => {
                let mut buff = String::new();
                for (k, v) in v.iter() {
                    buff.push_str(&format!("{}: {}", k, v));
                }
                f.write_str(&buff)
            }
            Type::HashMapS(v) => {
                let mut buff = String::new();
                for (k, v) in v.iter() {
                    buff.push_str(&format!("{}: {}", k, v));
                }
                f.write_str(&buff)
            }
            Type::FileOrData(ford) => match ford {
                FileOrData::File(file_path, _) => write!(
                    f,
                    "File: {}",
                    if let Some(path) = file_path.to_str() {
                        path
                    } else {
                        "Cannot parse path!"
                    }
                ),
                FileOrData::Bytes(b) => b.fmt(f),
            },
            Type::Any(_) => f.write_str("Any"),
            Type::CustomEnum(e) => {
                if let Some(e) = e.get_active() {
                    f.write_str(&e)
                } else {
                    f.write_str("None")
                }
            }
            Type::AdvancedEnum(_) => {
                f.write_str("Not Implemented")
                // if let Some(e) = e.get_active() {
                //     e
                // } else {
                //     format!("None")
                // }
            }
            Type::Vec(v) => v.fmt(f),
            Type::Bytes(b) => (b as &dyn std::fmt::Debug).fmt(f),
            Type::None => f.write_str(""),
        }
    }
}

impl From<u8> for Type {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}

impl From<u16> for Type {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}

impl From<u32> for Type {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<u64> for Type {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl From<u128> for Type {
    fn from(value: u128) -> Self {
        Self::U128(value)
    }
}

impl From<usize> for Type {
    fn from(value: usize) -> Self {
        Self::USize(value)
    }
}

impl From<i8> for Type {
    fn from(value: i8) -> Self {
        Self::I8(value)
    }
}

impl From<i16> for Type {
    fn from(value: i16) -> Self {
        Self::I16(value)
    }
}

impl From<i32> for Type {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<i64> for Type {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<i128> for Type {
    fn from(value: i128) -> Self {
        Self::I128(value)
    }
}

impl From<isize> for Type {
    fn from(value: isize) -> Self {
        Self::ISize(value)
    }
}

impl From<f32> for Type {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl From<f64> for Type {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}

impl From<bool> for Type {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<String> for Type {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl TryInto<u8> for Type {
    type Error = ();

    fn try_into(self) -> Result<u8, Self::Error> {
        if let Self::U8(value) = self {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl TryInto<u16> for Type {
    type Error = ();

    fn try_into(self) -> Result<u16, Self::Error> {
        if let Self::U16(value) = self {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl TryInto<u32> for Type {
    type Error = ();

    fn try_into(self) -> Result<u32, Self::Error> {
        if let Self::U32(value) = self {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl TryInto<u64> for Type {
    type Error = ();

    fn try_into(self) -> Result<u64, Self::Error> {
        if let Self::U64(value) = self {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl TryInto<u128> for Type {
    type Error = ();

    fn try_into(self) -> Result<u128, Self::Error> {
        if let Self::U128(value) = self {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl TryInto<usize> for Type {
    type Error = ();

    fn try_into(self) -> Result<usize, Self::Error> {
        if let Self::USize(value) = self {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl TryInto<i8> for Type {
    type Error = ();

    fn try_into(self) -> Result<i8, Self::Error> {
        if let Self::I8(value) = self {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl TryInto<i16> for Type {
    type Error = ();

    fn try_into(self) -> Result<i16, Self::Error> {
        if let Self::I16(value) = self {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl TryInto<i32> for Type {
    type Error = ();

    fn try_into(self) -> Result<i32, Self::Error> {
        if let Self::I32(value) = self {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl TryInto<i64> for Type {
    type Error = ();

    fn try_into(self) -> Result<i64, Self::Error> {
        if let Self::I64(value) = self {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl TryInto<i128> for Type {
    type Error = ();

    fn try_into(self) -> Result<i128, Self::Error> {
        if let Self::I128(value) = self {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl TryInto<isize> for Type {
    type Error = ();

    fn try_into(self) -> Result<isize, Self::Error> {
        if let Self::ISize(value) = self {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl TryInto<f32> for Type {
    type Error = ();

    fn try_into(self) -> Result<f32, Self::Error> {
        if let Self::F32(value) = self {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl TryInto<f64> for Type {
    type Error = ();

    fn try_into(self) -> Result<f64, Self::Error> {
        if let Self::F64(value) = self {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl TryInto<bool> for Type {
    type Error = ();

    fn try_into(self) -> Result<bool, Self::Error> {
        if let Self::Bool(value) = self {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl TryInto<String> for Type {
    type Error = ();

    fn try_into(self) -> Result<String, Self::Error> {
        if let Self::String(value) = self {
            Ok(value)
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
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
    FileOrData,

    Any,

    CustomEnum(CustomEnum),
    AdvancedEnum(AdvanceEnum),

    Vec(Box<TypeTag>),
    Bytes,

    None,
}

impl Display for TypeTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TypeTag::U8 => "u8",
            TypeTag::U16 => "u16",
            TypeTag::U32 => "u32",
            TypeTag::U64 => "u64",
            TypeTag::U128 => "u128",
            TypeTag::USize => "usize",
            TypeTag::I8 => "i8",
            TypeTag::I16 => "i16",
            TypeTag::I32 => "i32",
            TypeTag::I64 => "i64",
            TypeTag::I128 => "i128",
            TypeTag::ISize => "isize",
            TypeTag::F32 => "f32",
            TypeTag::F64 => "f64",
            TypeTag::Bool => "bool",
            TypeTag::String => "string",
            TypeTag::Url => "url",
            TypeTag::Path => "path",
            TypeTag::HashMapSS => "hashmap_string_string",
            TypeTag::HashMapS(h) => return write!(f, "hashmap_string({})", h),
            TypeTag::FileOrData => "file_or_data",
            TypeTag::Any => "any",
            TypeTag::CustomEnum(_) => "custom_enum",
            TypeTag::AdvancedEnum(_) => "advanced_enum",
            TypeTag::Vec(v) => return write!(f, "vec({})", v),
            TypeTag::Bytes => "bytes",
            TypeTag::None => "none",
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub enum TypeValidation {
    Range(usize, usize),
}
