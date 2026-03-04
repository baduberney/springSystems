fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();

    let mut max_word = "";
    let mut max_count = 0;

    for i in 0..words.len() {
        let current_word = words[i];
        let mut count = 0;

        for j in 0..words.len() {
            if words[j] == current_word {
                count += 1;
            }
        }

        if count > max_count {
            max_count = count;
            max_word = current_word;
        }
    }

    (max_word.to_string(), max_count)
}

fn main() {
    let text = "the fast red fox hops over the lazy dog the yellow red turtle the the";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}