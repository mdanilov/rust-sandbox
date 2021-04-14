fn main() {
    let _x = 5; // statement
    let y = {
        let x = 3;
        x + 1 // expression (no semicolon)
    };
    println!("The value of y is: {}", y);

    another_function(5, 6);

    let x = five();
    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5 // functions return the last expression
}
