fn most_frequent_word(text: &str) -> (String, usize) {
    let mut max_word = "";
    let mut max_count : usize = 0;

    for word in text.split_whitespace(){
        let mut count : usize = 0;
        for w in text.split_whitespace(){
            if w == word {
                count += 1;
            }
        }
        if count > max_count{
            max_count = count;
            max_word = word;
        }
    }

    (max_word.to_string(), max_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}