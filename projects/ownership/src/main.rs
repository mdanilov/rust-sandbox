fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    {
        let _s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    let x = 5;
    let _y = x; // Copy

    let s1 = String::from("hello");
    let _s2 = s1; // Move, s1 no longer valid

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Clone (deep copy)
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // x would be move inot function,
                   // but i32 is Copy, so it's okay to still use x afterwards

    let _s1 = gives_ownership(); // gives_ownership moves its return value into s1
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}
