pub fn heap_bytes(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = heap_bytes(2, 2);
        assert_eq!(result, 4);
    }
}
