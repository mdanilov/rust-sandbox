use std::fmt::Debug;
use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;

    // default implementation
    // can call other methods in the same trait
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
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

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify_short_form(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_2_short_form(_item: &(impl Summary + Display)) {}

pub fn notify_2<T: Summary + Display>(_item: &T) {}

fn some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
    0
}

fn some_function_with_where<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
