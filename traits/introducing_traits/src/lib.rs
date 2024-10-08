pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String{
        format!("(Read more from {})", self.summarize_author()) 
    }
}

pub struct NewsArticle {
    pub headline: String, 
    pub location: String, 
    pub author: String, 
    pub content: String,
}

//impl Summary for NewsArticle{ we can use defalt impl for all types by leaving this blank as below
//    fn summarize(&self) -> String {
//        format!("{}, by {} ({})", self.headline, self.author, self.location)
//    }
//}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{} is the author of 97 other articles", self.author)
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//impl Summary for Tweet{
//    fn summarize(&self) -> String {
//        format!("{}: {}", self.username, self.content)
//    }
//}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
