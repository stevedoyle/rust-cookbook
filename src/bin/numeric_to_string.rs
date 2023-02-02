
/// Convert a numeric value to a string.

fn main() {
    // Integer to string
    let x = 1234;
    let s: String = x.to_string();
    assert_eq!("1234", s);

    // Float to string
    let x = 12.34;
    let s: String = x.to_string();
    assert_eq!("12.34", s);

    // Integer to hex string
    let x = 0x1234;
    let s: String = format!("{:X}", x);
    assert_eq!("1234", s);
}