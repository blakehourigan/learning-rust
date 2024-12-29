pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, r2: &Rectangle) -> bool {
        if (self.width * self.height) > (r2.width * r2.height) {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adder_functionality() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_can_hold() {
        let larger = Rectangle {
            width: 10,
            height: 5,
        };

        let smaller = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
