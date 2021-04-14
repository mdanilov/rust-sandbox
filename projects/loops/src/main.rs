fn main() {
    // `loop`
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // returning values from the loop
        }
    };

    println!("The result is {}:", result);

    // `while`
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // `for`
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is {}:", element);
    }

    for number in (1..4).rev() {
        println!("The value is {}:", number);
    }
}
