pub fn size(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = size(2, 2);
        assert_eq!(result, 4);
    }
}
