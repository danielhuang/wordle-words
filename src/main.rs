use std::{fmt::Debug, sync::Arc, time::Instant};

use cached::proc_macro::cached;

use defaultmap::DefaultHashMap;
use itertools::Itertools;
use pathfinding::prelude::dfs;

#[derive(Default, PartialEq, Eq, Hash, Clone)]
struct LetterSet {
    set: [bool; 26],
}

impl LetterSet {
    fn pop(&mut self) -> Option<char> {
        for (letter, val) in ('a'..='z').zip(self.set.iter_mut()) {
            if *val {
                *val = false;
                return Some(letter);
            }
        }
        None
    }
}

impl FromIterator<char> for LetterSet {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut result = Self::default();
        for letter in iter {
            assert!(('a'..='z').contains(&letter));
            let i = (letter as usize) - ('a' as usize);
            result.set[i] = true;
        }
        result
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Word {
    word: [char; 5],
}

impl Word {
    fn new(c: &str) -> Word {
        let word = c.chars().collect_vec();
        Self {
            word: word.try_into().unwrap(),
        }
    }

    fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.word.iter().copied()
    }
}

impl Debug for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let word: String = self.chars().collect();
        write!(f, "Word({:?})", word)
    }
}

#[cached]
fn words() -> Vec<Word> {
    let mut w = include_str!("words.txt")
        .lines()
        .filter(|x| x.chars().all_unique())
        .unique_by(|x| x.chars().sorted().collect_vec())
        .map(Word::new)
        .collect_vec();

    let mut letter_frequency = DefaultHashMap::new(0);
    for word in &w {
        for letter in word.chars() {
            letter_frequency[letter] += 1;
        }
    }

    w.sort_unstable_by_key(|x| x.chars().map(|c| letter_frequency[c]).sum::<i32>());

    w
}

#[cached]
fn words_without(letters: LetterSet) -> Arc<Vec<Word>> {
    let mut letters = letters;
    if let Some(letter) = letters.pop() {
        Arc::new(
            words_without(letters)
                .iter()
                .filter(|x| !x.word.contains(&letter))
                .copied()
                .collect(),
        )
    } else {
        Arc::new(words())
    }
}

fn main() {
    let start = Instant::now();
    let x = dfs(
        vec![],
        |words: &Vec<Word>| {
            let previous_letters = words.iter().flat_map(|x| x.chars());
            words_without(previous_letters.collect())
                .iter()
                .filter(|x| {
                    if let Some(prev) = words.last() {
                        *x > prev
                    } else {
                        true
                    }
                })
                .map(|&x| {
                    let mut words = words.clone();
                    words.push(x);
                    words
                })
                .collect_vec()
        },
        |words| words.len() == 5,
    );
    dbg!(&x);
    dbg!(start.elapsed());
}
