mod poc1 {

    pub struct Deterministic;
    pub struct NonDeterministic;

    pub trait HeapBytesResolver<S> {
        fn count_bytes(&self) -> usize;
    }

    impl HeapBytesResolver<Deterministic> for String {
        fn count_bytes(&self) -> usize {
            self.len()
        }
    }

    impl HeapBytesResolver<NonDeterministic> for String {
        fn count_bytes(&self) -> usize {
            self.capacity()
        }
    }

    impl HeapBytesResolver<Deterministic> for i32 {
        fn count_bytes(&self) -> usize {
            0
        }
    }

    impl HeapBytesResolver<NonDeterministic> for i32 {
        fn count_bytes(&self) -> usize {
            0
        }
    }

    pub trait HeapBytes {
        fn heap_bytes<S>(&self) -> usize
        where
            Self: HeapBytesResolver<S>,
        {
            HeapBytesResolver::<S>::count_bytes(self)
        }
    }

    impl<T: ?Sized> HeapBytes for T {}
}

#[test]
fn poc() {
    use poc1::HeapBytes;

    struct MyStruct {
        field1: String,
        field2: i32,
    }

    impl<S> poc1::HeapBytesResolver<S> for MyStruct
    where
        String: poc1::HeapBytesResolver<S>,
        i32: poc1::HeapBytesResolver<S>,
    {
        fn count_bytes(&self) -> usize {
            self.field1.heap_bytes::<S>() + self.field2.heap_bytes::<S>()
        }
    }

    let mut my_struct = MyStruct {
        field1: String::with_capacity(10),
        field2: 42,
    };
    my_struct.field1.push_str("hello");

    assert_eq!(my_struct.heap_bytes::<poc1::Deterministic>(), 5);
    assert_eq!(my_struct.heap_bytes::<poc1::NonDeterministic>(), 10);
}

#[test]
fn new_strategy() {
    use poc1::HeapBytes;

    pub struct MyStrategy;

    impl poc1::HeapBytesResolver<MyStrategy> for String {
        fn count_bytes(&self) -> usize {
            self.capacity() * 10
        }
    }

    impl poc1::HeapBytesResolver<MyStrategy> for i32 {
        fn count_bytes(&self) -> usize {
            10
        }
    }

    struct MyStruct {
        field1: String,
        field2: i32,
    }

    impl<S> poc1::HeapBytesResolver<S> for MyStruct
    where
        String: poc1::HeapBytesResolver<S>,
        i32: poc1::HeapBytesResolver<S>,
    {
        fn count_bytes(&self) -> usize {
            self.field1.heap_bytes::<S>() + self.field2.heap_bytes::<S>()
        }
    }

    let mut my_struct = MyStruct {
        field1: String::with_capacity(10),
        field2: 42,
    };
    my_struct.field1.push_str("hello");

    assert_eq!(my_struct.heap_bytes::<poc1::Deterministic>(), 5);
    assert_eq!(my_struct.heap_bytes::<poc1::NonDeterministic>(), 10);
    assert_eq!(my_struct.heap_bytes::<MyStrategy>(), 110);
}
