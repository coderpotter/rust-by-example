fn main() {
    let s = String::from("hello world");
    let _hello = &s[0..5]; // or &s[..5]
    let _world = &s[6..11]; // or &s[6..]
    let word = first_word(&s);
    println!("The first word is: {}", word);
}
fn first_word(s: &String) -> &str {
    // returns a slice
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..] // or &s
}
