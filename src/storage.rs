use std::any::Any;

#[derive(Debug, Default)]
pub struct Storage {
    data: Vec<Box<dyn Any + Send + Sync>>,
}

impl Storage {
    pub fn push<T: Sync + Send + 'static>(&mut self, data: T) {
        self.data.push(Box::new(data))
    }

    pub fn get<T: Sync + Send + 'static>(&self, index: usize) -> Option<&T> {
        if let Some(data) = self.data.get(index) {
            data.downcast_ref::<T>()
        } else {
            None
        }
    }

    pub fn get_mut<T: Sync + Send + 'static>(&mut self, index: usize) -> Option<&mut T> {
        if let Some(data) = self.data.get_mut(index) {
            data.downcast_mut::<T>()
        } else {
            None
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<Box<dyn Any>> {
        if self.data.len() > index {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    pub fn insert<T: Sync + Send + 'static>(&mut self, index: usize, data: T) {
        self.data.insert(index, Box::new(data))
    }

    pub fn pop(&mut self) -> Option<Box<dyn Any + Send + Sync>> {
        self.data.pop()
    }

    pub fn iter(&self) -> std::slice::Iter<Box<dyn Any + Send + Sync>> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Box<dyn Any + Send + Sync>> {
        self.data.iter_mut()
    }
}
