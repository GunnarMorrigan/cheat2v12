use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Word<'a>{
    word: &'a str,
    letters: [u8; 26],
    contains_blanks: bool,
    min_value: u32,
    max_value: u32,
}

impl<'a> Display for Word<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.word)
    }
}

impl<'a> Word<'a>{
    pub fn new_from_str(word: &'a str) -> Option<Self>{
        let mut letters = [0u8; 26];
        let mut contains_blanks = false;
        let mut min_value = 0;
        let mut max_value = 0;

        for char in word.chars(){
            let mut digit = char as u8;
            if digit == 32{
                contains_blanks = true;
                max_value += 25;
            }
            else if digit >= 97 && digit <= 122 {  
                digit -= 97;
                letters[digit as usize] += 1;
                min_value += digit as u32;
                max_value += digit as u32;
            }
            else{
                return None;
            }
        }

        Some(Self{
            word,
            letters,
            contains_blanks,
            min_value,
            max_value,
        })
    }

    pub fn min_value(&self) -> u32{
        self.min_value
    }

    pub fn max_value(&self) -> u32{
        self.max_value
    }

    pub fn contains(&self, other: &Word) -> bool{
        let mut ret = false;

        for i in 0..26{
            if self.letters[i] < other.letters[i] {
                return false;
            }
        }
        true
    }

    pub fn all_letters_known(&self) -> bool{
        !self.contains_blanks
    }
}