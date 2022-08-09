#[macro_export]
macro_rules! add {
    ($($a:expr),+) => (0$(+$a)+)
}

#[macro_export]
macro_rules! mul {
    ($($a:expr),+) => (1$(*$a)+)
}

#[macro_export]
macro_rules! sub {
    ($a:expr, $b:expr) => {
        $a - $b
    };
}

#[macro_export]
macro_rules! div {
    ($a:expr, $b:expr) => {
        $a / $b
    };
}

#[macro_export]
macro_rules! eq {
    ($a:expr, $b:expr) => {
        $a == $b
    };
}

#[macro_export]
macro_rules! gt {
    ($a:expr, $b:expr) => {
        $a > $b
    };
}

#[macro_export]
macro_rules! lt {
    ($a:expr, $b:expr) => {
        $a < $b
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_add() {
        assert_eq!(15, add!(1, 2, 3, 4, 5));
        assert_eq!(1, add!(1));
    }

    #[test]
    fn test_mul() {
        assert_eq!(120, mul!(1, 2, 3, 4, 5));
        assert_eq!(1, mul!(1));
    }

    #[test]
    fn test_sub() {
        assert_eq!(5, sub!(10, 5));
    }

    #[test]
    fn test_div() {
        assert_eq!(2, div!(10, 5));
    }

    #[test]
    fn test_eq() {
        assert_eq!(false, eq!(10, 5));
        assert_eq!(true, eq!(10, 10));
    }

    #[test]
    fn test_lt() {
        assert_eq!(false, lt!(10, 5));
        assert_eq!(true, lt!(10, 11));
    }

    #[test]
    fn test_gt() {
        assert_eq!(true, gt!(10, 5));
        assert_eq!(false, gt!(10, 11));
    }
}
