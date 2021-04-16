fn main() {
    let mut _s = String::new();
    let data = "initial contents";
    _s = data.to_string();
    // the method also works on a literal directly:
    let _s = "initial contents".to_string();
    let _s = String::from("initial contents");

    // strings are UTF-8 encoded
    let _hello = String::from("Hola");
    let _hello = String::from("Здравствуйте");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let _s = format!("{}-{}-{}", s1, s2, s3);

    // iterates over symbols
    for c in "Здравствуйте".chars() {
        println!("{}", c);
    }

    // iterates over bytes
    for b in "Здравствуйте".bytes() {
        println!("{}", b);
    }
}
