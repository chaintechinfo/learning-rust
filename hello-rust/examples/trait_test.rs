use std::fmt::{Display, Formatter, Result};

trait Summary {
    fn summarize(&self) -> String;

    fn summarize_default(&self) -> String {
        String::from(format!("Read more... {}", self.summarize_author()))
    }

    fn summarize_author(&self) -> String;
}

struct NewsArticle {
    headline: String,
    localtion: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        println!("{}", self.content);

        format!("{}, by {} ({})", self.headline, self.author, self.localtion)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} - {} - {} - {}", self.username, self.content, self.reply, self.retweet)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

fn notify2<T: Summary>(item: &T) {
    println!("Bound trait -> {}", item.summarize());
}

fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!("{} {}", item1.summarize(), item2.summarize_default());
}

fn notify4<T: Summary>(item1: &T, item2: &T) {
    println!("{} {}", item1.summarize(), item2.summarize_default());
}

fn notify5(item: &(impl Summary + Display)) {
    println!("{}, and it's summary is {}", item, item.summarize());
}

fn notify6<T: Summary + Display>(item: &T) {
    println!("{}, and it's summary is {}", item, item.summarize());
}

fn main() {
    let tweet = Tweet { 
        username: String::from("ebook"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("Tweet summarize: {}", tweet.summarize());
    println!("Default: {}", tweet.summarize_default());

    notify(&tweet);
    notify2(&tweet);

    notify3(&tweet, &tweet);
    notify4(&tweet, &tweet);

    notify5(&tweet);
    notify6(&tweet);
}