struct User {
    active: bool,
    username: String,
    email: String, 
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn main() {
    tuple_structs();
}

//fn user_example(){
//    let mut user1 = User {
//        active: true,
//        username:String::from("ducklover69"),
//        email: String::from("ducklover69@420mail.net"),
//        sign_in_count: 64,
//    };
//
//    user1.email = String::from("duckhater69@420mail.net");
//
//    let user2 = build_user(String::from("suckducks420@69.net"),String::from("duckssuck"));
//
//    println!("user 1 details:");
//    println!("{}, {}, {}, {}", user1.active, user1.username, user1.email, user1.sign_in_count);
//    println!(" "); 
//
//    println!("user 2 details:");
//    println!("{}, {}, {}, {}", user2.active, user2.username, user2.email, user2.sign_in_count);
//
//    // struct update syntax allows us to use most values of another instance, while updating a few
//    // values
//
//    let user3 = User{
//        email: String::from("yourmom@420.net"),
//        ..user2
//    };
//
//    println!("user 3 details:");
//    println!("{}, {}, {}, {}", user3.active, user3.username, user3.email, user3.sign_in_count);
//}
//fn build_user(email: String, username: String) -> User{ this uses standard initialization
//    User{                                         however we can use 'field init' shorthand
//        active: true,                             found below to avoid repeating struct attribute
//        username: username,                       names that are the same as the function names 
//        email: email,
//        sign_in_count: 1, 
//    }
//}

//fn build_user(email:String, username: String) -> User{
//    User{                                           // this gives the same functionality but is 
//        active: true,                               // much more concise
//        username,
//        email, 
//        sign_in_count: 1,
//    }
//}

fn tuple_structs(){
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    
    println!("{}, {}, {}",black.0, black.1, black.2);
    println!("{}, {}, {}",origin.0, origin.1, origin.2); 
}
