mod word;

use std::{fs, collections::{HashMap, HashSet}, time::Instant, env};

use word::Word;



fn main() {
    let words = include_str!("wordlist.txt");

    let args: Vec<String> = env::args().collect();
    // args = dbg!(args);

    let searchword = Word::new_from_str(args[1].as_str()).unwrap();

    let mut wordmap: HashMap<u32, Vec<Word>> = HashMap::new();

    // words.make_ascii_lowercase();
    let mut word_count = 0;
    for dict_word in words.lines(){
        if dict_word.len() == 12{
            match Word::new_from_str(dict_word) {
                Some(dict_word) => {
                    word_count += 1;
                    match wordmap.get_mut(&dict_word.min_value()) {
                        Some(wordlist) => {
                            wordlist.push(dict_word);
                        },
                        None => {
                            wordmap.insert(dict_word.min_value(), vec![dict_word]);
                        },
                    }
                },
                None => (),
            }
        }
    }
    println!("Total number of 12 letter words: {}", word_count);
    let time = Instant::now();

    if searchword.all_letters_known(){
        search_one_option(searchword, wordmap);
    }
    else{
        search_wide_range(searchword, wordmap);
    }
    println!("{:?}", time.elapsed());
    // let words = wordmap.get(&searchwordvalue).unwrap();
}

pub fn search_one_option(search_word: Word, mut words: HashMap<u32, Vec<Word>>){
    let mut possiblewords = Vec::new();
    let list = words.remove(&search_word.min_value()).unwrap();
    for word in list{
        if word.contains(&search_word){
            possiblewords.push(word);
        }
    }
    for word in possiblewords{
        println!("{}", word);
    }
}

pub fn search_wide_range(search_word: Word, words: HashMap<u32, Vec<Word>>){
    let mut possible_words = Vec::new();
    for possible_value in search_word.min_value()..=search_word.max_value(){
        if let Some(word_list) = words.get(&possible_value){
            for word in word_list{
                if word.contains(&search_word){
                    possible_words.push(word);
                }
            }
        }
    }
    for word in possible_words{
        println!("{}", word);
    }
}