use std::collections::HashSet;

pub struct Tokenizer {
    vocabulary: HashSet<Vec<u8>>,
}

impl Tokenizer {
    fn init_bpe_vocab() -> HashSet<Vec<u8>> {
        let mut vocabulary: HashSet<Vec<u8>> = HashSet::new();
        for utf in 0..=255_u8 {
            vocabulary.insert(vec![utf]);
        }

        return vocabulary;
    }

    pub fn new() -> Self {
        Self {
            vocabulary: Self::init_bpe_vocab(),
        }
    }

    pub fn from_value(vocabulary: HashSet<Vec<u8>>) -> Self {
        Self {
            vocabulary: vocabulary,
        }
    }

    pub fn encode(&self, corpus: String) -> Vec<Vec<u8>> {
        // behavior : aabbcd, voc = [a, aa,b, c, d, bc]
        // tokenized : [aa, b, bc, d]
        // pass 1 : a look in voc, a exists, tokenize with a,
        // pass 2 : aa look in voc, aa exists, tokenize with aa,
        // pass 3: aab look in voc, aab doesn't exist, tokenize with aa, reset pointer to current,
        // b exists tokenize with b
        // pass 4 : bb, look in voc, bb do
        // pass 5
        let mut byte_seq: Vec<u8> = Vec::new();
        let mut tokens: Vec<Vec<u8>> = Vec::new();
        for element in corpus.as_bytes() {
            byte_seq.push(element.clone());
            if byte_seq.len() > 0 && self.vocabulary.contains(&byte_seq) {
                continue;
            }
            let token = byte_seq[..byte_seq.len() - 1].to_vec();
            tokens.push(token);
            byte_seq = byte_seq[byte_seq.len() - 1..].to_vec();
        }

        if byte_seq.len() > 0 {
            // dump last token
            tokens.push(byte_seq.clone());
        }

        return tokens;
    }

    // fn train(corpus: String, num_rounds: u32) {
    //     let mut tokenized_corpus: Vec<Vec<u8>> = Vec::new();
    //     // TODO : complete the function after calling encode
    // }
}

// fn compute_frequencies(tokenized_corpus: Vec<Vec<u8>>) -> HashMap<Vec<Vec<u8>>, u8> {
//     let mut frequencies: HashMap<Vec<Vec<u8>>, u8> = HashMap::new();
//     let mut prev_token: Vec<Vec<u8>> = Vec::new();
//     for element in &tokenized_corpus {
//         prev_token.push(element.clone());
//         if prev_token.len() == 2 {
//             *frequencies.entry(prev_token.clone()).or_insert(0) += 1;
//             prev_token.clear();
//             prev_token.push(element.clone());
//         }
//     }
//
//     return frequencies;
// }
//
// fn train_bpe(corpus: String, num_rounds: u32) {
//     let vocabulary = init_bpe_vocab();
//
//     for element in corpus.as_bytes() {
//         tokenized_corpus.push(vec![*element]);
//     }
//
//     for round in 0..num_rounds {
//         println!("[Training] Round {}", round);
//         let frequencies = compute_frequencies(tokenized_corpus);
//         for (key, value) in frequencies {
//             let token: Vec<u8> = key.into_iter().flatten().collect();
//             let s = String::from_utf8(token).unwrap();
//             println!("For token : {}, frequency = {}", s, value);
//         }
//     }
// }
