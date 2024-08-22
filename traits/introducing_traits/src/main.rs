use introducing_traits::{Summary, Tweet, NewsArticle};

pub fn notify<T>(item: &T)
where
    T: Summary,
{
    println!("Breaking news!: {}", item.summarize());
}



fn main() {
    let tweet = Tweet {
        username: String::from("blanket"),
        content: String::from(
            "what's the deal with airline food?"),
            reply: true, 
            retweet: true,
    };

    println!("1 new tweet: {}", tweet.summarize());

    println!(" ");

    let article = NewsArticle{
        location: String::from("covid-19 shuts down nba finals"),
        author: String::from("luca doncic"),
        content: String::from("covid-19 has shut down the nba playoffs."),
    };

    println!("1 new article: {}", article.summarize());

    notify(&article);


}



