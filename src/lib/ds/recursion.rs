pub fn factorial(n: usize) -> usize {
    (1..=n).product()
}

#[cfg(test)]
mod should_correctly_recurse {
    use super::*;
    #[test]
    fn factorial_test() {
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(6), 720);
    }

    #[test]
    fn iterator_playground() {
        let range = 0..=4;
        assert_eq!(&range.size_hint(), &(5, Some(5)));
        assert_eq!(&range.clone().max().unwrap(), &4);
        assert_eq!(range.max_by(|x, y| x.cmp(y)).unwrap(), 4);
    }
}
