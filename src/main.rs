use rand;
use std::collections::HashMap;
struct RandomizedSet {
    arraylist: Vec<i32>,
    hashmap: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            arraylist: Vec::new(),
            hashmap: HashMap::new(),
        }
    }
    fn insert(&mut self, val: i32) -> bool {
        if self.arraylist.contains(&val) {
            return false;
        }
        self.hashmap.insert(val, self.arraylist.len() as i32);
        self.arraylist.push(val);
        true
    }
    fn remove(&mut self, val: i32) -> bool {
        if self.arraylist.contains(&val) {
            let last_element = self.arraylist[self.arraylist.len()-1];
            if let Some(idx) = self.hashmap.get(&val) {
                self.arraylist[*idx as usize] = last_element;
                self.hashmap.insert(last_element, *idx);
                self.hashmap.remove_entry(&val);
                self.arraylist.pop();
                return true;
            }
        }
        false
    }
    fn get_random(&self) -> i32 {
        let x = rand::random::<usize>();
        self.arraylist[x % self.arraylist.len()]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
fn main() {
    let test = vec![vec![0, 3], vec![4, 8], vec![3, 5]];
}
