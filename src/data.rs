use serde::{Deserialize, Serialize};

use crate::{
    prelude::{TypeTag, TypeValidation},
    types::Type,
};

use bytes_kman::TBytes;

use std::{
    collections::HashMap,
    fs::File,
    hash::Hash,
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[derive(Debug, Default, Clone, Serialize, Deserialize, Hash, bytes_kman::Bytes)]
pub struct Bytes {
    pub data: Vec<u8>,
    pub coursor: usize,
    pub fast_invert: bool,
}

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Self {
            coursor: value.len(),
            data: value,
            fast_invert: false,
        }
    }
}
impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        Self {
            coursor: value.len(),
            data: value.to_vec(),
            fast_invert: false,
        }
    }
}

impl std::io::Write for Bytes {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = buf.len();
        for b in buf {
            if self.data.len() == self.coursor {
                self.data.push(*b);
            } else {
                self.data[self.coursor] = *b;
                self.coursor += 1;
            }
        }
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl std::io::Read for Bytes {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut readed = 0;
        for byte in buf.iter_mut() {
            if self.coursor == self.data.len() {
                break;
            }
            if self.coursor == 0 {
                break;
            }

            *byte = if self.fast_invert {
                self.data[self.data.len() - self.coursor]
            } else {
                self.data[self.coursor]
            };

            readed += 1;
            self.coursor += 1;

            if self.coursor == self.data.len() {
                break;
            }
            if self.coursor == 0 {
                break;
            }
        }
        Ok(readed)
    }
}

impl std::io::Seek for Bytes {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        match pos {
            std::io::SeekFrom::Start(pos) => {
                let res = pos;
                if res >= self.data.len() as u64 {
                    Err(std::io::Error::from_raw_os_error(25))
                } else {
                    self.coursor = res as usize;
                    Ok(res)
                }
            }
            std::io::SeekFrom::End(pos) => {
                let res = (self.data.len() as i64) + pos;
                if res >= self.data.len() as i64 {
                    Err(std::io::Error::from_raw_os_error(24))
                } else {
                    self.coursor = res as usize;
                    Ok(res as u64)
                }
            }
            std::io::SeekFrom::Current(pos) => {
                let res = pos + self.coursor as i64;
                if res >= self.data.len() as i64 {
                    Err(std::io::Error::from_raw_os_error(25))
                } else {
                    self.coursor = res as usize;
                    Ok(res as u64)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, bytes_kman::Bytes)]
pub struct Value {
    pub value: Type,
    pub should_be: Vec<TypeTag>,
    pub validators: Vec<TypeValidation>,
    pub default: Type,
    pub desc: String,
    pub editabile: bool,
}

impl Value {
    pub fn new(
        value: Type,
        should_be: Vec<TypeTag>,
        validators: Vec<TypeValidation>,
        editabile: bool,
        desc: impl Into<String>,
    ) -> Self {
        Self {
            value: value.clone(),
            default: value,
            should_be,
            validators,
            desc: desc.into(),
            editabile,
        }
    }
}

impl From<Type> for Value {
    fn from(value: Type) -> Self {
        Self {
            value: value.clone(),
            should_be: vec![value.to_tag()],
            validators: vec![],
            default: value,
            desc: String::new(),
            editabile: true,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, bytes_kman::Bytes)]
pub struct Data {
    pub data: HashMap<String, Value>,
    pub locked: bool,
}

impl Hash for Data {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.locked.hash(state);
        for (k, v) in self.data.iter() {
            k.hash(state);
            v.hash(state)
        }
    }
}

impl Data {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            locked: false,
        }
    }

    pub fn set(&mut self, key: &str, value: Type) -> Option<Type> {
        let Some(data) = self.data.get_mut(key) else{
            return None;
        };

        let mut value = value;
        std::mem::swap(&mut data.value, &mut value);
        Some(value)
    }

    pub fn reset(&mut self, key: &str) -> Option<Type> {
        let Some(data) = self.data.get(key)else{return None};

        self.set(key, data.default.clone())
    }

    pub fn get(&self, key: &str) -> Option<&Type> {
        let Some(data) = self.data.get(key) else{
            return None;
        };
        Some(&data.value)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Type> {
        let Some(data) = self.data.get_mut(key) else{
            return None;
        };
        Some(&mut data.value)
    }

    /// if has a result that's mean that has an error!
    pub fn validate(&self) -> Option<String> {
        let mut errors = String::new();

        for (key, value) in self.iter() {
            let mut has_correct_type = false;
            for should_be in value.should_be.iter() {
                let t = value.value.to_tag();
                if *should_be == t {
                    has_correct_type = true;
                    break;
                }
            }

            if !has_correct_type {
                let mut buff = format!("`{key}` should be: ");
                for (i, should_be) in value.should_be.iter().enumerate() {
                    if i > 0 {
                        buff.push(',');
                    }
                    buff.push_str(&should_be.to_string());
                }
                errors.push_str(&buff);
            }

            // TODO: Implement validators!
            //let mut is_valid = true;

            if !has_correct_type
            /* | !is_valid */
            {
                return Some(errors);
            }
        }
        None
    }

    pub fn get_value(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    pub fn get_mut_value(&mut self, key: &str) -> Option<&mut Value> {
        self.data.get_mut(key)
    }

    /// should not be used!
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.data.remove(key)
    }

    /// only should be used by the module
    pub fn add(&mut self, key: &str, value: impl Into<Value>) {
        if !self.locked {
            self.data.insert(key.to_owned(), value.into());
        }
    }

    /// only should be used by module
    pub fn lock(&mut self) {
        self.locked = true;
    }

    /// should not be used!!!
    pub fn unlock(&mut self) {
        self.locked = false;
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Value> {
        self.data.iter()
    }

    /// you should only modify value
    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<String, Value> {
        self.data.iter_mut()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileOrData {
    File(PathBuf, #[serde(skip)] Option<Arc<Mutex<std::fs::File>>>),
    Bytes(Bytes),
}

impl TBytes for FileOrData {
    fn size(&self) -> usize {
        match self {
            FileOrData::File(p, _) => p.size() + 1,
            FileOrData::Bytes(v) => v.size() + 1,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        match self {
            FileOrData::File(path, _) => {
                let mut buff = Vec::with_capacity(self.size());

                buff.push(0);
                buff.append(&mut path.to_bytes());

                buff
            }
            FileOrData::Bytes(bytes) => {
                let mut buff = Vec::with_capacity(self.size());

                buff.push(1);
                buff.append(&mut bytes.to_bytes());

                buff
            }
        }
    }

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let variant = buffer.pop()?;

        match variant {
            0 => Some(Self::File(PathBuf::from_bytes(buffer)?, None)),
            1 => Some(Self::Bytes(Bytes::from_bytes(buffer)?)),
            _ => None,
        }
    }
}

impl Hash for FileOrData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            FileOrData::File(f, _) => f.hash(state),
            FileOrData::Bytes(b) => b.hash(state),
        }
    }
}

impl std::io::Write for FileOrData {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            FileOrData::File(file_path, file) => {
                if let Some(file) = file {
                    file.lock().unwrap().write(buf)
                } else {
                    let mut f = File::options()
                        .create(true)
                        .write(true)
                        .read(true)
                        .open(file_path)?;
                    let res = f.write(buf);
                    *file = Some(Arc::new(Mutex::new(f)));
                    res
                }
            }
            FileOrData::Bytes(bytes) => bytes.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            FileOrData::File(_, file) => {
                if let Some(file) = file {
                    file.lock().unwrap().flush()
                } else {
                    Ok(())
                }
            }
            FileOrData::Bytes(_) => Ok(()),
        }
    }
}

impl std::io::Read for FileOrData {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            FileOrData::File(file_path, file) => {
                if let Some(file) = file {
                    file.lock().unwrap().read(buf)
                } else {
                    let mut f = File::options().read(true).write(true).open(file_path)?;
                    let res = f.read(buf);
                    *file = Some(Arc::new(Mutex::new(f)));
                    res
                }
            }
            FileOrData::Bytes(bytes) => bytes.read(buf),
        }
    }
}

impl std::io::Seek for FileOrData {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        match self {
            FileOrData::File(file_path, file) => {
                if let Some(file) = file {
                    file.lock().unwrap().seek(pos)
                } else {
                    let mut f = File::options().read(true).write(true).open(file_path)?;
                    let res = f.seek(pos);
                    *file = Some(Arc::new(Mutex::new(f)));
                    res
                }
            }
            FileOrData::Bytes(bytes) => bytes.seek(pos),
        }
    }
}
