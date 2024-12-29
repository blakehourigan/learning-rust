use traits::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("blake"),
        content: String::from("i just drank a fly..."),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.summarize());

    println!("{:?}", traits::notify(&tweet));
}
