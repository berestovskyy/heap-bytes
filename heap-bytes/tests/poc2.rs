mod poc2 {
    pub struct Deterministic;
    pub struct NonDeterministic;

    pub trait HeapBytes<S> {
        fn heap_bytes(&self) -> usize {
            0
        }
    }

    impl HeapBytes<Deterministic> for String {
        fn heap_bytes(&self) -> usize {
            self.len()
        }
    }
    impl HeapBytes<NonDeterministic> for String {
        fn heap_bytes(&self) -> usize {
            self.capacity()
        }
    }
    impl HeapBytes<Deterministic> for i32 {}
    impl HeapBytes<NonDeterministic> for i32 {}
}

#[test]
fn new_strategy() {
    pub struct MyStrategy;

    impl poc2::HeapBytes<MyStrategy> for String {
        fn heap_bytes(&self) -> usize {
            self.capacity() * 10
        }
    }
    impl poc2::HeapBytes<MyStrategy> for i32 {
        fn heap_bytes(&self) -> usize {
            10
        }
    }

    struct MyStruct {
        field1: String,
        field2: i32,
    }

    impl<S> poc2::HeapBytes<S> for MyStruct
    where
        String: poc2::HeapBytes<S>,
        i32: poc2::HeapBytes<S>,
    {
        fn heap_bytes(&self) -> usize {
            self.field1.heap_bytes() + self.field2.heap_bytes()
        }
    }

    let mut my_struct = MyStruct {
        field1: String::with_capacity(10),
        field2: 42,
    };
    my_struct.field1.push_str("hello");

    assert_eq!(
        poc2::HeapBytes::<poc2::Deterministic>::heap_bytes(&my_struct),
        5
    );
    assert_eq!(
        poc2::HeapBytes::<poc2::NonDeterministic>::heap_bytes(&my_struct),
        10
    );
    assert_eq!(poc2::HeapBytes::<MyStrategy>::heap_bytes(&my_struct), 110);
}
