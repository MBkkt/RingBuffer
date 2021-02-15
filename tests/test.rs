#[cfg(test)]
mod tests {
    extern crate lib;

    use lib::RingBuffer;
    use std::{
        collections::VecDeque
    };

    #[test]
    fn it_works() {
        let mut their = VecDeque::new();
        let mut my = RingBuffer::new();
        their.reserve(3);
        my.with_capacity(3);
        their.push_back(10);
        their.push_back(11);
        their.push_back(12);
        my.push(10);
        my.push(11);
        my.push(12);
        assert_eq!(my.pop(), their.pop_front());
        assert_eq!(my.pop(), their.pop_front());
        assert_eq!(my.pop(), their.pop_front());
        assert_eq!(my.pop(), their.pop_front());
    }
}