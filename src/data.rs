use get_ref::TGetRef;

use crate::types::Type;

use std::{
    any::TypeId,
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct Bytes(pub Vec<u8>);

#[allow(unused)]
pub trait Fields {
    fn get_fields(&self) -> Vec<&str>;

    fn is_struct(&self) -> bool;
    fn get_field<T: 'static>(&self, id: usize) -> Option<&T> {
        None
    }
    fn get_field_mut<T: 'static>(&mut self, id: usize) -> Option<&mut T> {
        None
    }
    fn get_field_type(&self, id: usize) -> Option<TypeId> {
        None
    }

    fn is_enum(&self) -> bool;
    fn enum_is(&self, id: usize) -> bool {
        false
    }
    fn enum_get_type(&self, id: usize, tuple: usize) -> Option<TypeId> {
        None
    }

    /// if return is `usize::MAX` is means that a error has accured!
    fn enum_tuple_len(&self, id: usize) -> usize {
        usize::MAX
    }
    fn enum_get<T: 'static>(&self, tuple_id: usize) -> Option<&T> {
        None
    }
    fn enum_get_mut<T: 'static>(&mut self, tuple_id: usize) -> Option<&mut T> {
        None
    }
    fn enum_set<T: 'static + TGetRef>(&mut self, id: usize, value: T) -> Option<()> {
        None
    }

    /// if return is `usize::MAX` is means that a error has accured!
    fn enum_current(&self) -> usize {
        usize::MAX
    }
}

// #[derive(Debug, Clone)]
// pub enum DataGlob {
//     I8(i8),
//     I16(i16),
//     I32(i32),
//     I64(i64),
//     I128(i128),
//     ISize(isize),

//     U8(u8),
//     U16(u16),
//     U32(u32),
//     U64(u64),
//     U128(u128),
//     USize(usize),

//     F32(f32),
//     F64(f64),

//     Bool(bool),

//     CustomEnum(CustomEnum),
//     AdvanceEnum(AdvanceEnum),

//     String(String),
//     Path(PathBuf),
//     HashMap(HashMap<String, String>),
//     Bytes(Vec<u8>),
//     TcpStrean(Arc<Mutex<TcpStream>>),
//     TcpListener(Arc<Mutex<TcpListener>>),
//     Any(Arc<Mutex<Box<dyn Any>>>),

//     FileOrData(FileOrData),

//     OptionI8(Option<i8>),
//     OptionI16(Option<i16>),
//     OptionI32(Option<i32>),
//     OptionI64(Option<i64>),
//     OptionI128(Option<i128>),
//     OptionISize(Option<isize>),

//     OptionU8(Option<u8>),
//     OptionU16(Option<u16>),
//     OptionU32(Option<u32>),
//     OptionU64(Option<u64>),
//     OptionU128(Option<u128>),
//     OptionUSize(Option<usize>),

//     OptionF32(Option<f32>),
//     OptionF64(Option<f64>),

//     OptionBool(Option<bool>),

//     OptionString(Option<String>),
//     OptionPath(Option<PathBuf>),
//     OptionHashMap(Option<HashMap<String, String>>),
//     OptionTcpStream(Option<Arc<Mutex<TcpStream>>>),
//     OptionTcpListener(Option<Arc<Mutex<TcpListener>>>),

//     OptionFileOrData(Option<FileOrData>),

//     None,
// }

#[derive(Debug, Clone, Default)]
pub struct Data {
    pub data: HashMap<String, Type>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: Type) -> Option<Type> {
        self.data.insert(key.to_owned(), value)
    }

    pub fn get(&self, key: &str) -> Option<&Type> {
        self.data.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Type> {
        self.data.get_mut(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<Type> {
        self.data.remove(key)
    }

    pub fn default(&mut self, key: &str, value: Type) {
        if let None = self.data.get(key) {
            self.data.insert(key.to_owned(), value);
        }
    }

    pub fn iter(&mut self) -> std::collections::hash_map::Iter<String, Type> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<String, Type> {
        self.data.iter_mut()
    }
}

#[derive(Debug, Clone)]
pub enum FileOrData {
    File(PathBuf, Option<Arc<Mutex<std::fs::File>>>),
    Bytes(Bytes),
}
