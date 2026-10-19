pub fn print_tokenized_bytes(tokens: Vec<Vec<u8>>) {
    for elt in tokens {
        let txt_token = String::from_utf8(elt).unwrap();
        println!("{}", txt_token);
    }
}
