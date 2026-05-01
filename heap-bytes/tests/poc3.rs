mod poc3 {
    pub struct Deterministic;
    pub struct NonDeterministic;
    pub trait Strategy {}
    impl Strategy for Deterministic {}
    impl Strategy for NonDeterministic {}

    pub trait Is<U> {}
    impl<U> Is<U> for U {}

    pub trait HeapBytes<T: Strategy> {
        fn heap_bytes<F>(&self) -> usize
        where
            F: Strategy + Is<T>;
    }

    impl HeapBytes<Deterministic> for String {
        fn heap_bytes<F>(&self) -> usize {
            self.len()
        }
    }

    impl HeapBytes<NonDeterministic> for String {
        fn heap_bytes<F>(&self) -> usize {
            self.capacity()
        }
    }

    impl HeapBytes<Deterministic> for i32 {
        fn heap_bytes<F>(&self) -> usize {
            0
        }
    }

    impl HeapBytes<NonDeterministic> for i32 {
        fn heap_bytes<F>(&self) -> usize {
            0
        }
    }
}

#[test]
fn new_strategy() {
    struct MyStrategy;
    impl poc3::Strategy for MyStrategy {}

    impl poc3::HeapBytes<MyStrategy> for String {
        fn heap_bytes<F>(&self) -> usize {
            self.capacity() * 10
        }
    }
    impl poc3::HeapBytes<MyStrategy> for i32 {
        fn heap_bytes<F>(&self) -> usize {
            10
        }
    }

    struct MyStruct {
        field1: String,
        field2: i32,
    }

    impl<M: poc3::Strategy> poc3::HeapBytes<M> for MyStruct
    where
        String: poc3::HeapBytes<M>,
        i32: poc3::HeapBytes<M>,
    {
        fn heap_bytes<F>(&self) -> usize
        where
            F: poc3::Strategy + poc3::Is<M>,
        {
            self.field1.heap_bytes::<F>() + self.field2.heap_bytes::<F>()
        }
    }

    let mut my_struct = MyStruct {
        field1: String::with_capacity(10),
        field2: 42,
    };
    my_struct.field1.push_str("hello");

    use poc3::HeapBytes;

    assert_eq!(my_struct.heap_bytes::<poc3::Deterministic>(), 5);
    assert_eq!(my_struct.heap_bytes::<poc3::NonDeterministic>(), 10);
    assert_eq!(my_struct.heap_bytes::<MyStrategy>(), (10 * 10) + 10);
}
