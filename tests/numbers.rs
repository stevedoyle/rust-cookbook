#[cfg(test)]
mod tests {

    #[test]
    // Conversion from decimal strings to integers
    fn from_int_strings() {
        // First approach: Use `string.parse()`.
        let num = "1234".parse().unwrap();
        assert_eq!(1234, num);
    }

    #[test]
    // Conversion from hex strings to an integer
    fn from_hex_strings() {
        // First approach: Use `i32::from_str_radix`. The hex string must not begin
        // with "0x". Note that string.parse() doesn't work for hex strings. It only
        // works for decimal strings. Instead `i32::from_str_radix()` must be used.
        let num = i32::from_str_radix("12AB", 16).unwrap();
        assert_eq!(0x12AB, num);

        // Second approach: Using the parse_int crate. It requires that the hex
        // string begins with "0x".
        let s = String::from("0x12AB");
        let num: i32 = parse_int::parse(&s).unwrap();
        assert_eq!(0x12AB, num);

        // Third approach: Using the hex crate, convert into a Vec<u8> and then
        // into an integer by reconstructing the integer from the vector elements.
        // In this approach, the hex string must not begin with "0x".
        let raw_bytes = hex::decode("12AB").unwrap();
        let mut num: i32 = 0;
        for val in raw_bytes {
            num = (num << 8) + i32::from(val);
        }
        assert_eq!(0x12AB, num);
    }

    #[test]
    // Conversion from strings to a float
    fn from_float_strings() {
        // First approach: Use `string.parse()`.
        let num = "12.34".parse().unwrap();
        assert_eq!(12.34, num);
    }

    #[test]
    fn numeric_to_string() {
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
}