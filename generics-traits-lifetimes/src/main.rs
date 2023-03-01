use std::fmt::Display;
fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    let mut largest: Option<&T> = None;

    for item in list {
        largest = match largest {
            Some(num) if item > num => Some(item),
            Some(_) => largest,
            None => Some(item),
        };
    }

    largest
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        return format!("Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Headline: {}\nAuthor: {}\n", self.headline, self.author)
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{} ({})", self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub struct BlogArticle {
    pub title: String,
    pub content: String,
    pub author: String,
}

impl Summary for BlogArticle {
    fn summarize_author(&self) -> String {
        format!("By {}", self.author)
    }
}

// fn notify(item: &impl Summary) -> () {
fn notify<T: Summary + Display>(item: &T) -> () {
    println!("Notification: {}", item.summarize())
}

fn main() {
    let numbers = vec![1, 2, 43, 3, 100, 2];
    println!("Largest number: {:?}", largest(&numbers));
    let chars = vec!['a', 'b', 't', 'a', 'c'];
    println!("Largest char: {:?}", largest(&chars));

    let tweet = Tweet {
        username: String::from("primalivet"),
        content: String::from("Hello world"),
        reply: false,
        retweet: false
    };
    let newsarticle = NewsArticle {
        headline: String::from("A new article"),
        content: String::from("Hello world"),
        author: String::from("primalivet"),
        location: String::from("Sweden"),
    };
    let blogarticle = BlogArticle {
        title: String::from("Hello world"),
        content: String::from("content of blog article"),
        author: String::from("primalivet"),
    };

    // notify(&tweet);
    notify(&newsarticle);
    // notify(&blogarticle);

    // println!("{}", tweet.summarize());
    // println!("{}", newsarticle.summarize());
    // println!("{}", blogarticle.summarize());
}
