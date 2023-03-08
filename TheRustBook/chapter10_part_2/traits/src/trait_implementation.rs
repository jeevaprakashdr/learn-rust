use crate::trait_as_parameter;

pub struct NewsArticle {
    pub author : String,
    pub headline : String,
    pub content : String
}

impl Summary for NewsArticle {
    
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

impl Display for NewsArticle {
    
    fn disp(&self) -> String {
        format!("{}, by {}", self.author, self.headline)
    }
}

pub struct Tweet {
    pub username : String,
    pub content : String,
    pub reply : bool,
    pub retweet : bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }
}

impl Display for Tweet {
    fn disp(&self) -> String {
        format!("{}, by {}", self.content, self.username)
    }
}


pub trait Summary {
    fn summarize(&self) -> String;
} 

pub trait Display {
    fn disp(&self) -> String;
} 


pub fn demo() {
    let article = NewsArticle{
        author: String::from("Bin Navis"),
        content: String::from("lagadhiya"),
        headline: String::from("notice"),
    };

    let tweet = Tweet {
        content:String::from("Shock Laga"),
        username: String::from("chamarprash"),
        reply:false,
        retweet: false
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("News Article summary: {}", article.summarize());

    trait_as_parameter::notify(&tweet);
}