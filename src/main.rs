use std::{fmt::Debug, time::Instant};

use itertools::Itertools;

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
struct LetterSet {
    set: u32,
}

impl LetterSet {
    fn add(self, other: Self) -> Self {
        Self {
            set: self.set | other.set,
        }
    }
}

impl FromIterator<char> for LetterSet {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut result = Self::default();
        for letter in iter {
            assert!(('a'..='z').contains(&letter));
            let i = (letter as usize) - ('a' as usize);
            result.set |= 1 << i;
        }
        result
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Word {
    word: [u8; 5],
}

impl Word {
    fn new(c: &str) -> Word {
        let word = c.chars().map(|x| x as _).collect_vec();
        Self {
            word: word.try_into().unwrap(),
        }
    }

    fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.word.iter().copied().map(|x| x as _)
    }
}

impl Debug for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let word: String = self.chars().collect();
        write!(f, "Word({:?})", word)
    }
}

fn words() -> Vec<Word> {
    let start = Instant::now();
    let mut w = include_str!("words.txt")
        .lines()
        .filter(|x| x.chars().all_unique())
        .unique_by(|x| x.chars().sorted().collect_vec())
        .map(Word::new)
        .collect_vec();

    let mut letter_frequency = vec![0; 128];
    for word in &w {
        for letter in word.chars() {
            letter_frequency[letter as usize] += 1;
        }
    }

    dbg!(&letter_frequency.iter().max().unwrap());

    w.sort_unstable_by_key(|x| x.chars().map(|c| letter_frequency[c as usize]).sum::<i32>());

    dbg!(&start.elapsed());

    w
}

fn word_contains(word: Word, letters: LetterSet) -> bool {
    for c in word.chars() {
        for (i, letter) in ('a'..='z').enumerate() {
            if letter == c && letters.set & (1 << i) != 0 {
                return true;
            }
        }
    }

    false
}

fn words_without_from(words: &[Word], letters: LetterSet) -> Vec<Word> {
    words
        .iter()
        .filter(|&&x| !word_contains(x, letters))
        .copied()
        .collect()
}

fn search(
    words: &mut Vec<Word>,
    available_words: &Vec<Word>,
    avoid_letters: LetterSet,
) -> Option<Vec<Word>> {
    if words.len() == 5 {
        return Some(words.clone());
    }

    for &word in available_words {
        words.push(word);

        let avoid_next = word.chars().collect();
        let avoid_letters = avoid_letters.add(avoid_next);

        let available_words = words_without_from(available_words, avoid_letters);
        if let Some(word) = search(words, &available_words, avoid_letters) {
            return Some(word);
        }

        words.pop();
    }
    None
}

fn main() {
    let start = Instant::now();

    let mut w = vec![];
    let x = search(&mut w, &words(), LetterSet::default());
    dbg!(&x);
    dbg!(start.elapsed());
}
