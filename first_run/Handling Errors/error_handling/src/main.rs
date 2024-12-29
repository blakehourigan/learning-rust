use std::fs::File;
use std::io::ErrorKind;

fn main() {
    unwrap_and_expect();
}

//fn handling_file_errors_using_match(){
//    let greeting_file_result = File::open("hello.txt");
//
//    let greeting_file = match greeting_file_result{
//        Ok(file) => file, 
//        Err(error) => match error.kind(){
//            ErrorKind::NotFound => match File::create("hello.txt"){
//                Ok(fc) => fc, 
//                Err(e) => panic!("Problem creating the file {e:?}"),
//            },
//            other_error => {
//                panic!("Problem opening the file {other_error:?}"); 
//            }
//        },
//    };
//}

//fn handling_file_errors_using_closures() {
//    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
//        if error.kind() == ErrorKind::NotFound{
//            File::create("hello.txt").unwrap_or_else(|error|{
//                panic!("Problem creating the file {error:?}");
//            })
//        } else {
//            panic!("some other error occurred {error:?}"); 
//        }
//    });
//}

fn unwrap_and_expect() {
    //let greeting_file = File::open("hello.txt").unwrap();
   
    let greeting_file = File::open("hello.txt").expect("Error opening the file");  
}
