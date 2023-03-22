use std::{sync::{Mutex, Arc}};

pub struct Signal<T> {
    internal: Arc<Mutex<T>>,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            internal: self.internal.clone()
        }
    }
}

impl<T: Default + Clone> Signal<T> {
    pub fn new () -> Self {
        Self{
            internal: Arc::new(Mutex::new(T::default()))
        }
    }
    
    pub fn create_connection(&self) -> Connection<T>{
        Connection {
            internal: self.internal.clone()
        }
    }
}

pub struct Connection<T> {
    internal: Arc<Mutex<T>>,
}

impl<T: Default + Clone> Connection<T> {
    pub fn read_copy(&self) -> T {
        let inner_value = self.internal.lock().unwrap();
        return inner_value.clone();
    }
    
    pub fn write_copy(&self, val: T){
        let mut inner_value = self.internal.lock().unwrap();
        *inner_value = val
    }
}