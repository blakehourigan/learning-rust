fn main() {
    let config_max = Some(3u8);

    // match config_max {
    //     Some(max) => println!("the max value is {max}"),
    //     _ => (),
    // };

    // this just allows you to avoid boilerplate _ => ()
    if let Some(max) = config_max {
        println!("max is going to be {max}");
    };
}
