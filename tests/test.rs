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

    #[test]
    fn test_simple() {
        let mut tester = RingBuffer::new();
        tester.with_capacity(7);
        assert_eq!(tester.capacity(), 7);
        assert_eq!(tester.len(), 0);

        tester.push(1);
        tester.push(2);
        tester.push(3);
        tester.push(4);
        assert_eq!(tester.len(), 4);
        assert_eq!(tester.pop(), Some(1));
        assert_eq!(tester.pop(), Some(2));
        assert_eq!(tester.len(), 2);
        assert_eq!(tester.pop(), Some(3));
        assert_eq!(tester.pop(), Some(4));
        assert_eq!(tester.pop(), None);
    }

    #[test]
    fn test_simple_reversely() {
        let mut tester = RingBuffer::new();
        tester.with_capacity(7);
        assert_eq!(tester.capacity(), 7);
        assert_eq!(tester.len(), 0);

        tester.push(1);
        tester.push(2);
        tester.push(3);
        tester.push(4);
        assert_eq!(tester.len(), 4);
        assert_eq!(tester.pop(), Some(1));
        assert_eq!(tester.pop(), Some(2));
        assert_eq!(tester.len(), 2);
        assert_eq!(tester.pop(), Some(3));
        assert_eq!(tester.pop(), Some(4));
        assert_eq!(tester.pop(), None);
    }

    #[test]
    fn test_overflow_saturating() {
        let mut tester = RingBuffer::new();
        tester.with_capacity(2);
        assert_eq!(tester.push(1), true);
        assert_eq!(tester.push(2), true);
        assert_eq!(tester.push(3), false);
    }

    #[test]
    fn test_pop_empty() {
        let mut tester = RingBuffer::new();
        tester.with_capacity(1);
        assert_eq!(tester.push(1), true);
        assert_eq!(tester.pop(), Some(1));
        assert_eq!(tester.len(), 0);
        assert_eq!(tester.pop(), None);
    }

    #[test]
    fn test_index() {
        let mut tester = RingBuffer::new();
        tester.with_capacity(3);
        tester.push(1);
        tester.push(2);
        tester.push(3);
        // [1, 2, 3]
        assert_eq!(tester.pop(), Some(1));
        // [2, 3]
        assert_eq!(tester.len(), 2);
        // [2, 3, 0]
        tester.push(0);
        assert_eq!(tester.pop(), Some(2));
        // [3, 0]
    }

    #[test]
    fn test_into_iter() {
        #[derive(Eq, PartialEq, Debug)]
        struct NoCopy<T>(T);

        {
            let mut tester: RingBuffer<NoCopy<u8>> = RingBuffer::new();
            tester.with_capacity(2);
            tester.push(NoCopy(1));
            tester.push(NoCopy(2));
            let mut iter = tester.into_iter();
            assert_eq!(iter.size_hint(), (2, Some(2)));
            assert_eq!(iter.next(), Some(NoCopy(1)));
            assert_eq!(iter.next(), Some(NoCopy(2)));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.size_hint(), (0, Some(0)));
        }
        {
            let mut tester: RingBuffer<NoCopy<u8>> = RingBuffer::new();
            tester.with_capacity(2);
            tester.push(NoCopy(1));
            tester.push(NoCopy(2));
            tester.pop();
            tester.push(NoCopy(3));
            let mut iter = tester.into_iter();
            assert_eq!(iter.next(), Some(NoCopy(2)));
            assert_eq!(iter.next(), Some(NoCopy(3)));
            assert_eq!(iter.next(), None);
        }
        {
            let mut tester: RingBuffer<NoCopy<u8>> = RingBuffer::new();
            tester.with_capacity(2);
            tester.push(NoCopy(1));
            tester.push(NoCopy(2));
            tester.pop();
            tester.push(NoCopy(3));
            tester.pop();
            tester.push(NoCopy(4));
            let mut iter = tester.into_iter();
            assert_eq!(iter.next(), Some(NoCopy(3)));
            assert_eq!(iter.next(), Some(NoCopy(4)));
            assert_eq!(iter.next(), None);
        }
    }

    #[test]
    fn test_drop() {
        use std::cell::Cell;

        let flag = &Cell::new(0);

        struct Bump<'a>(&'a Cell<i32>);

        impl<'a> Drop for Bump<'a> {
            fn drop(&mut self) {
                let n = self.0.get();
                self.0.set(n + 1);
            }
        }

        {
            let mut tester = RingBuffer::new();
            tester.with_capacity(128);
            tester.push(Bump(flag));
            tester.push(Bump(flag));
        }
        assert_eq!(flag.get(), 2);

        // test something with the nullable pointer optimization
        flag.set(0);
        {
            let mut tester = RingBuffer::new();
            tester.with_capacity(3);
            tester.push(vec![Bump(flag)]);
            tester.push(vec![Bump(flag), Bump(flag)]);
            tester.push(vec![]);
            tester.push(vec![Bump(flag)]);
            assert_eq!(flag.get(), 1);
            drop(tester.pop());
            assert_eq!(flag.get(), 2);
            drop(tester.pop());
            assert_eq!(flag.get(), 4);
        }
        assert_eq!(flag.get(), 4);
    }

    #[test]
    fn test_partial_equal() {
        const CAP: usize = 10;
        let mut tester = RingBuffer::new();

        for len in 0..CAP + 1 {
            for padding in 0..CAP {
                tester.with_capacity(padding);

                let mut expected = RingBuffer::new();
                tester.with_capacity(CAP);
                for x in 0..len {
                    tester.push(x);
                    expected.push(x);
                }
                // assert_eq!(tester, expected);

                // test negative
                if len > 2 {
                    tester.pop();
                    expected.pop();
                    // assert!(tester != expected);
                }
            }
        }
    }
}