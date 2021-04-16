use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    {
        let x = 5;              // ----------+-- 'b
                                //           |
        let r = &x;             // --+-- 'a  |
                                //   |       |
        println!("r: {}", r);   //   |       |
                                // --+       |
                                // ----------+
    }

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // Lifetime elision with 3 rules:
    // 1. Each parameter that is a reference gets its own lifetime parameter.
    //    fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
    // 2. If there is exactly one input lifetime parameter,
    //    that lifetime is assigned to all output lifetime parameters
    //    fn foo<'a>(x: &'a i32) -> &'a i32
    // 3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`
    //    because this is a method, the lifetime of self is assigned to all output lifetime parameters.

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    i.level();
    let part = i.announce_and_return_part("hello");
    println!("Part is: {}", part);

    // static lifetime
    // means that this reference can live for the entire duration of the program.
    let _s: &'static str = "I have a static lifetime.";
}
