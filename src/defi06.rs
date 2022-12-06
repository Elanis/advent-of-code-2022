use std::collections::VecDeque;
use std::collections::HashSet;
use std::hash::Hash;

pub fn do_work(input : String) -> u32 {
    let letters = input.chars().collect::<Vec<char>>();

    let mut a = letters[0];
    let mut b = letters[1];
    let mut c = letters[2];
    let mut d = letters[3];

    for i in 4..letters.len() {
        if a != b && b != c && c != d && b != d && a != d && a != c {
            return (i).try_into().unwrap();
        }

        a = b;
        b = c;
        c = d;
        d = letters[i];
    }

    0
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

pub fn do_work_2(input : String) -> u32 {
    let letters = input.chars().collect::<Vec<char>>();

    let mut elements : VecDeque<char> = VecDeque::new();
    for i in 0..14 {
        elements.push_front(letters[i]);
    }

    for i in 14..letters.len() {
        if has_unique_elements(&elements) {
            return (i).try_into().unwrap();
        }

        elements.pop_back();
        elements.push_front(letters[i]);
    }

    letters.len().try_into().unwrap()
}