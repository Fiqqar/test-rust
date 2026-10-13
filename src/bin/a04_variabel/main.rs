fn main() {
    let nama = "fiqar";
    let mut programming_language = "Rust";
    let mut message_number = 1;
    println!("Message {}: Hello, {}! You are currently learning {}", message_number, nama, programming_language);

    message_number = 2;
    programming_language = "Python";
    println!("Message {}: Hello, {}! You are currently learning {}", message_number, nama, programming_language);

    message_number = 3;
    let nomor = 123;
    println!("Message {2}: Hello, {1}! You are currently learning {0}. Test {3}", programming_language, nama, message_number, nomor);
}
