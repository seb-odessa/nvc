use std::collections::HashMap;

pub trait NextValidCharacter {
    fn parse(&mut self, line: &str);
    fn children(&self) -> Vec<char>;
    fn next(&self, word: &str) -> Vec<char>;
}

#[derive(Debug)]
pub struct HashMapNvc {
    nvc: HashMap<char, Self>,
}
impl HashMapNvc {
    pub fn new() -> Self {
        Self {
            nvc: HashMap::new(),
        }
    }
}
impl NextValidCharacter for HashMapNvc {
    fn parse(&mut self, word: &str) {
        let mut chars = word.chars();
        if let Some(head) = chars.next() {
            let tail = chars.as_str();
            let nvc = self.nvc.entry(head).or_insert(Self::new());
            if !tail.is_empty() {
                nvc.parse(tail);
            }
        }
    }

    fn children(&self) -> Vec<char> {
        let mut result = self.nvc.keys().map(|x| *x).collect::<Vec<char>>();
        result.sort();
        return result;
    }

    fn next(&self, word: &str) -> Vec<char> {
        let mut chars = word.chars();
        if let Some(head) = chars.next() {
            if let Some(nvc) = self.nvc.get(&head) {
                let tail = chars.as_str();
                if tail.is_empty() {
                    return nvc.children();
                } else {
                    return nvc.next(tail);
                }
            }
        } else {
            return self.children();
        }
        return Vec::new();
    }
}

#[cfg(test)]
mod nvc {
    use super::*;

    #[test]
    fn init() {
        assert!(HashMapNvc::new().children().is_empty());
    }

    #[test]
    fn branches() {
        let mut nvc = HashMapNvc::new();
        branches_impl(&mut nvc);
    }

    fn branches_impl(nvc: &mut impl NextValidCharacter) {
        assert_eq!(nvc.children(), vec![]);

        nvc.parse("");
        nvc.parse("abra");
        nvc.parse("cadabra");
        assert_eq!(nvc.children(), vec!['a', 'c']);
    }

    #[test]
    fn next() {
        let mut nvc = HashMapNvc::new();
        next_impl(&mut nvc);
    }

    fn next_impl(nvc: &mut impl NextValidCharacter) {
        assert_eq!(nvc.children(), vec![]);

        nvc.parse("abra");
        nvc.parse("abre");
        nvc.parse("abru");
        nvc.parse("abca");
        nvc.parse("abcu");
        nvc.parse("acub");

        assert_eq!(nvc.next("z"), vec![]);
        assert_eq!(nvc.next(""), vec!['a']);
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
        let mut nvc = HashMapNvc::new();
        search_nvc_impl(&mut nvc);
    }

    fn search_nvc_impl(nvc: &mut impl NextValidCharacter) {
        assert_eq!(nvc.children(), vec![]);
        let lines = vec![
            "Заря над Араксом",
            "Заря над бездной",
            "Заря над бухтой",
            "Заря над Сибирью",
            "Заря над Скаргиаром",
            "Заря над степью",
            "Заря над Уссури",
        ];

        for line in &lines {
            nvc.parse(line);
        }

        for line in &lines {
            let mut path = String::with_capacity(line.len());
            let mut chars = line.chars();
            while let Some(ch) = chars.next() {
                let nvcs = nvc.next(&path);
                assert!(nvcs.contains(&ch));
                path.push(ch);
            }
        }
    }
}
