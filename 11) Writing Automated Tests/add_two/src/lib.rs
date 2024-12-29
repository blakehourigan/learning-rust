pub fn add_two(num: u64) -> u64 {
    num + 2
}

pub fn add_three(num: u64) -> u64 {
    num + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }
}

#[cfg(test)]
mod more {
    use super::*;

    #[test]
    fn add_three_and_three() {
        let result = add_three(3);
        assert_eq!(6, result);
    }
}
