pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool{
        if (self.width * self.height) > (other.width * other.height){
            true
        }else{
            false
        }
    }
}

pub fn add_two(x: i32) -> i32{
    x + 2 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 5, 
        };
        let smaller = Rectangle {
            width: 5,
            height: 2, 
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two(){
        let result = add_two(5);

        assert_eq!(result, 7);
    }
}
