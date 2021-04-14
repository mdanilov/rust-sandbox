fn main() {
    // Mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants
    const _MAX_POINTS: u32 = 100_000;

    // Shadowing
    let x = 5;
    let x = x + 1;
    let x = x + 2;
    println!("The value of x is: {}", x);

    let spaces = "  ";
    let _spaces = spaces.len();

    // Data Types

    // Integer Types
    // Length   | Signed    | Unsigned
    // ---------|-----------|-----------
    // 8-bit    | i8        | u8
    // 16-bit   | i16       | u16
    // 32-bit   | i32       | u32
    // 64-bit   | i64       | u64
    // 128-bit  | i128      | u128
    // arch     | isize     | usize

    let _n = 98_222; // Decimal
    let _n = 0xff; // Hex
    let _n = 0o77; // Octal
    let _n = 0b1111_0000; // Binary
    let _n = b'A'; // Byte (u8 only)

    let _n = 57u8; // type suffix

    // Floating Types
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Boolean Type
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // Character Type
    // 4 bytes in size and represents a Unicode Scalar Value
    let _c = 'z';

    // Compound Types

    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup; // destructuring

    println!("The value of y is: {}", y);

    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    // The Array Type
    // have fixed length, allocated on stack
    let _a = [1, 2, 3, 4, 5];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // same as let a = [3, 3, 3, 3, 3];

    let _first = a[0];
    let _second = a[1];
}
