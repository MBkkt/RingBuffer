use std::{
    vec::Vec,
    option::Option,
    mem::swap,
};

pub struct RingBuffer<T> {
    data: Vec<Option<T>>,
    begin: usize,
    end: usize,
}

impl<T> RingBuffer<T> {
    pub fn new() -> RingBuffer<T> {
        RingBuffer { data: Vec::new(), begin: 0, end: 0 }
    }

    pub fn with_capacity(&mut self, n: usize) {
        self.begin = 0;
        self.end = 0;
        self.data.resize_with(n, Option::default);
    }

    pub fn push(&mut self, value: T) -> bool {
        assert!(self.begin <= self.end);
        let len = self.data.len();
        if self.end - self.begin == len {
            return false;
        }
        self.data[self.end % len] = Some(value);
        self.end += 1;
        return true;
    }

    pub fn pop(&mut self) -> Option<T> {
        assert!(self.begin <= self.end);
        let mut value = None::<T>;
        if self.begin != self.end {
            let len = self.data.len();
            swap(&mut value, &mut self.data[self.begin % len]);
            self.begin += 1;
        }
        return value;
    }
}

