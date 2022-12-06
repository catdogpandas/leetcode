/*
 * @lc app=leetcode.cn id=208 lang=rust
 *
 * [208] 实现 Trie (前缀树)
 */

// @lc code=start
#[derive(Default)]
struct Trie([Option<Box<Trie>>; 26], bool);

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        word.into_bytes()
            .into_iter()
            .map(|c| (c - b'a') as usize)
            .fold(self, |t, i| t.0[i].get_or_insert_with(Default::default))
            .1 = true
    }
    fn find(&self, prefix: String) -> (bool, bool) {
        let mut t = self;
        for i in prefix.into_bytes().into_iter().map(|c| (c - b'a') as usize) {
            match &t.0[i] {
                Some(s) => t = &s,
                _ => return (false, false),
            }
        }
        (true, t.1)
    }

    fn search(&self, word: String) -> bool {
        self.find(word).1
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.find(prefix).0
    }
}

// @lc code=end
