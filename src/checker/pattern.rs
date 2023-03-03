
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
pub struct Pattern <T>
where
    T: Eq + Hash + Clone
{
    pub pattern: Vec<T>,
    pub size: usize,
    pub indices: HashMap<T, usize>,
}

impl <T> Pattern <T> 
where
    T: Eq + Hash + Clone
{
    pub fn new(list:Vec<T>, hash: bool) -> Self {

        if hash {
            let mut indices = HashMap::new();
            for i in 0..list.len() {
                indices.insert((list[i]).clone(), i);
            }
            return Pattern {
                pattern: list.clone(),
                size: list.len(),
                indices,
            }
        }
        Pattern {
            pattern: list.clone(),
            size: list.len(),
            indices: HashMap::new(),
        }
    }

    pub fn get_equality(&self, other: &Pattern<T>) -> bool {
        for i in 0..self.size {
            if self.pattern[i] != other.pattern[i] {
                return false;
            }
        }
        true
    }
}

// fn main() {
//     let arr = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
//     let mut pattern = Pattern::new(arr, true);
//     println!("{:?}", pattern.indices);

//     let arr2 = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "1"];
//     let mut pattern2 = Pattern::new(arr2, false);

//     println!("{}", pattern.get_equality(&pattern2));

    // println!("{:?}", pattern.indices.get("1"));
    // println!("{:?}", pattern.indices.get("2"));
    // println!("{}", pattern.indices.get(String::from("3")));
    // println!("{}", pattern.indices.get(String::from("4")));
    // println!("{}", pattern.indices.get(String::from("")));
    // println!("{}", pattern.indices.get(String::from()));
// }