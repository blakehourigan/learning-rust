use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt")
        // .unwrap()
        .expect("hello.txt should be included in the proejct files");

    // most programmers choose expect over unwrap, because it can provide a greater deal of
    // context.

    // let greeting_file_result = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("error creating file: `hello.txt'. error: {error}");
    //         })
    //     } else {
    //         panic!("other error {error}");
    //     }
    // });

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(open_error) => panic!("error creating hello.txt: {open_error:?}"),
    //         },
    //         other_error => panic!("error accessing file: {other_error}"),
    //     },
    // };
}
