use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Nvc {
    nvc: HashMap<char, Nvc>,
}
impl Nvc {
    pub fn new() -> Self {
        Self {
            nvc: HashMap::new(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut chars = word.chars();
        if let Some(head) = chars.next() {
            let tail = chars.as_str();
            let nvc = self.nvc.entry(head).or_insert(Self::new());
            if !tail.is_empty() {
                nvc.insert(tail);
            }
        }
    }

    pub fn insert_filtered(&mut self, word: &str, allowed: &HashSet<char>) {
        let mut chars = word.chars();
        if let Some(head) = chars.next() {
            if allowed.contains(&head) {
                let tail = chars.as_str();
                let nvc = self.nvc.entry(head).or_insert(Self::new());
                if !tail.is_empty() {
                    nvc.insert_filtered(tail, allowed);
                }
            }
        }
    }

    fn branches(&self) -> Vec<char> {
        let mut result = self.nvc.keys().map(|x| *x).collect::<Vec<char>>();
        result.sort();
        return result;
    }

    pub fn next(&self, word: &str) -> Vec<char> {
        let mut chars = word.chars();
        if let Some(head) = chars.next() {
            if let Some(nvc) = self.nvc.get(&head) {
                let tail = chars.as_str();
                if tail.is_empty() {
                    return nvc.branches();
                } else {
                    return nvc.next(tail);
                }
            }
        }
        return Vec::new();
    }
}

#[cfg(test)]
mod nvc {
    use super::*;

    #[test]
    fn init() {
        assert!(Nvc::new().branches().is_empty());
    }

    #[test]
    fn branches() {
        let mut nvc = Nvc::new();
        assert_eq!(nvc.branches(), vec![]);

        nvc.insert("");
        nvc.insert("abra");
        nvc.insert("cadabra");
        assert_eq!(nvc.branches(), vec!['a', 'c']);
    }

    #[test]
    fn next() {
        let mut nvc = Nvc::new();

        nvc.insert("abra");
        nvc.insert("abre");
        nvc.insert("abru");
        nvc.insert("abca");
        nvc.insert("abcu");
        nvc.insert("acub");

        assert_eq!(nvc.next("z"), vec![]);
        assert_eq!(nvc.next("a"), vec!['b', 'c']);
        assert_eq!(nvc.next("ab"), vec!['c', 'r']);
        assert_eq!(nvc.next("abr"), vec!['a', 'e', 'u']);
        assert_eq!(nvc.next("abra"), vec![]);
        assert_eq!(nvc.next("ac"), vec!['u']);
        assert_eq!(nvc.next("acu"), vec!['b']);
        assert_eq!(nvc.next("abc"), vec!['a', 'u']);
    }
}
