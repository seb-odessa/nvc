use std::collections::HashMap;

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
        } else {
            return self.branches();
        }
        return Vec::new();
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn init() {
        assert!(!Nvc::new().branches().is_empty());
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

        assert_eq!(false, true);

        // assert_eq!(nvc.next(""), vec!['a', 'b']);
        assert_eq!(nvc.next("z"), vec!['a']);
        assert_eq!(nvc.next("a"), vec!['b', 'c']);
        assert_eq!(nvc.next("ab"), vec!['c', 'r']);
        assert_eq!(nvc.next("abr"), vec!['a', 'e', 'u']);
        assert_eq!(nvc.next("abra"), vec![]);
        assert_eq!(nvc.next("ac"), vec!['u']);
        assert_eq!(nvc.next("acu"), vec!['b']);
        assert_eq!(nvc.next("abc"), vec!['a', 'u']);
    }

    #[test]
    fn search_nvc() {
        let lines = vec![
            "Заря над Араксом",
            "Заря над Араксом",
            "Заря над бездной",
            "Заря над бухтой",
            "Заря над Сибирью",
            "Заря над Скаргиаром",
            "Заря над степью",
            "Заря над Уссури",
        ];
        let mut nvc = Nvc::new();
        for line in &lines {
            nvc.insert(line);
        }

        for line in &lines {
            let mut path = String::with_capacity(line.len());
            let chars = line.chars();
            while let Some(ch) = chars.next() {

            }

        }
    }
}
