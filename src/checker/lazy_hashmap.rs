// implementation of a lazy hashmap

use std::collections::HashMap;
use std::hash::Hash;

pub struct LazyHashMap<K, V, F>
where
    K: Eq + Hash + Clone,
    F: Fn(&K) -> V,
{
    map: HashMap<K, V>,
    f: F,
}

impl<K, V, F> LazyHashMap<K, V, F>
where
    K: Eq + Hash + Clone,
    F: Fn(&K) -> V,
{
    pub fn new(f: F) -> Self {
        LazyHashMap {
            map: HashMap::new(),
            f,
        }
    }

    pub fn get(&mut self, key: &K) -> &V {
        if !self.map.contains_key(key) {
            self.map.insert((*key).clone(), (self.f)(key));
        }
        self.map.get(key).unwrap()
    }
}

// fn main() {

//     let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

//     //get the index of 5
//     let mut map = LazyHashMap::new(|&x| arr.iter().position(|&y| y == x).unwrap());
//     println!("{}", map.get(&1));
//     println!("{}", map.get(&2));
//     println!("{}", map.get(&3));
//     println!("{}", map.get(&4));
//     println!("{}", map.get(&5));
//     println!("{}", map.get(&6));
// }