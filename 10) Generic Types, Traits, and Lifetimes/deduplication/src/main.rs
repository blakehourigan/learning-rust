fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest_num = largest(&number_list);
    println!("the largest number in the vector is {largest_num}");

    let number_list = vec![
        43, 32, 1999232, 423432, 12, 12332, 134324, 34234, 342434343, 43,
    ];

    let largest_num = largest(&number_list);
    println!("the largest number in the vector is {largest_num}");

    let char_list = vec!['v', 'c', 'b', 'a', 'e', 'y'];
    let largest_character = largest(&char_list);

    println!("the largest char in the list is: {largest_character}");
}

fn largest_i32(number_list: &[i32]) -> &i32 {
    let mut largest = &number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_char(char_list: &[char]) -> &char {
    let mut largest = &char_list[0];

    for character in char_list {
        if character > largest {
            largest = character;
        }
    }
    largest
}

// the above code is very redundant, to combat this, we can use GENERICS as below...

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
