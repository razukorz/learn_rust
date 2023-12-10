fn main() {
    let long_string: String = String::from("LAI IS COME QU IS GO");

    let first_word: &str = first_word(&long_string);

    println!("First word is {}.", first_word);
}

fn first_word(s: &str) -> &str {
    // make the string slice into a byte slice, each character is a byte
    let bytes= s.as_bytes();

    // iterate through the byte slice and find the index of the first whitespace
    // return a string slice between the start of the string and the first whitespace
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    // no whitespace found, the string slice is word itself
    // return a string slice of the whole string
    return &s[..];
}
