mod tokenizer;
mod utils;
use std::collections::HashSet;
use tokenizer::Tokenizer;
use utils::print_tokenized_bytes;

fn main() {
    let vocabulary: HashSet<Vec<u8>> = [
        b"a".to_vec(),
        b"aa".to_vec(),
        b"b".to_vec(),
        b"c".to_vec(),
        b"d".to_vec(),
        b"bc".to_vec(),
    ]
    .into_iter()
    .collect();
    let corpus = String::from("aabbcd");
    let t = Tokenizer::from_value(vocabulary);
    let tokens = t.encode(corpus);

    print_tokenized_bytes(tokens);
}
