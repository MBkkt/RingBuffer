use std::{
    vec::Vec,
    option::Option,
    mem::swap,
};

pub struct RingBuffer<T> {
    data: Vec<Option<T>>,
    begin: usize,
    size: usize,
}

impl<T> RingBuffer<T> {
    pub fn new() -> RingBuffer<T> {
        RingBuffer { data: Vec::new(), begin: 0, size: 0 }
    }

    pub fn with_capacity(&mut self, n: usize) {
        self.begin = 0;
        self.size = 0;
        self.data.resize_with(n, Option::default);
    }

    pub fn push(&mut self, value: T) -> bool {
        let len = self.data.len();
        if self.size == len {
            return false;
        }
        self.data[(self.begin + self.size) % len] = Some(value);
        self.size += 1;
        return true;
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut value = None::<T>;
        if self.size != 0 {
            let len = self.data.len();
            swap(&mut value, &mut self.data[self.begin]);
            self.begin = (self.begin + 1) % len;
            self.size -= 1;
        }
        return value;
    }
}

