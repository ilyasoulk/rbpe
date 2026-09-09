use std::collections::{HashMap, HashSet};

// basic algorithm :
// - Start with a base vocab (all UTF-8 values 0 - 255)
// - Iterate on the corpus byte per byte
// - Keep the longer running known sequence in memory, if it's a know token then keep going
// if it's not a known token then we have completed the previous known token and we can start
// constructing the next new token as the new pair
fn train_bpe(corpus: String, num_rounds: u32) {
    let mut vocabulary: HashSet<Vec<u8>> = HashSet::new();
    for utf in 0..=255_u8 {
        vocabulary.insert(vec![utf]);
    }

    let mut tokenized_corpus: Vec<Vec<u8>> = Vec::new();

    for element in corpus.as_bytes() {
        tokenized_corpus.push(vec![*element]);
    }

    for round in 0..num_rounds {
        println!("[Training] Round {}", round);
        let mut frequencies: HashMap<Vec<Vec<u8>>, u8> = HashMap::new();
        let mut prev_token: Vec<Vec<u8>> = Vec::new();
        for element in &tokenized_corpus {
            prev_token.push(element.clone());
            if prev_token.len() == 2 {
                *frequencies.entry(prev_token.clone()).or_insert(0) += 1;
                prev_token.clear();
                prev_token.push(element.clone());
            }
        }

        for (key, value) in frequencies {
            let token: Vec<u8> = key.into_iter().flatten().collect();
            let s = String::from_utf8(token).unwrap();
            println!("For token : {}, frequency = {}", s, value);
        }
    }
}

fn main() {
    let corpus = String::from("abababc");
    let num_rounds = 1;
    train_bpe(corpus, num_rounds);
}
