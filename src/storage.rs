#![allow(dead_code)]

use std::any::Any;

#[derive(Debug, Default)]
pub struct Storage {
    pub data: Vec<Box<dyn Any + Send>>,
}

impl Storage {
    pub fn get<T: 'static>(&self) -> Option<&T> {
        for element in self.data.iter() {
            if let Some(data) = element.downcast_ref::<T>() {
                return Some(data);
            }
        }

        None
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        for element in self.data.iter_mut() {
            if let Some(data) = element.downcast_mut::<T>() {
                return Some(data);
            }
        }

        None
    }

    pub fn set<T: 'static + Send>(&mut self, data: T) -> Option<Box<T>> {
        let res = self.remove();
        self.data.push(Box::new(data));
        res
    }

    pub fn remove<T: 'static>(&mut self) -> Option<Box<T>> {
        let mut finded = None;
        for (i, element) in self.data.iter().enumerate() {
            if element.downcast_ref::<T>().is_some() {
                finded = Some(i);
                break;
            }
        }

        if let Some(finded) = finded {
            Some(self.data.remove(finded).downcast::<T>().unwrap())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Data {
        id: usize,
    }

    #[test]
    fn storage() {
        let mut storage = Storage::default();

        let res = storage.set(Data { id: 21 });

        assert!(res.is_none());

        assert_eq!(storage.get::<Data>().unwrap().id, 21);

        let res = storage.set(Data { id: 92 });

        assert!(res.is_some());

        assert_eq!(res.unwrap().id, 21);

        assert_eq!(storage.get::<Data>().unwrap().id, 92);
    }
}
