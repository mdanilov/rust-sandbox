fn main() {
    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3]; // &[i32]

    let my_string = String::from("hello world");

    let _word = first_word(&my_string);

    // first_word works on slices of `String`s
    let _word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let _word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
