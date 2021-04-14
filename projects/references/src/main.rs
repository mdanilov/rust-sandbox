fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);

    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems

    let _r2 = &mut s;

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}.", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    no_dangle();
}

fn calculate_length(s: &String) -> usize {
    // s is a referenece to a String
    s.len()
} // Here, s goes out of scope. But because it does not have the ownership to  what
  // it refers to, nothing happens

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s // ownership is moved out
}
