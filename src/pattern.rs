/*
    Pattern:
        pattern size: int
        pattern: array
        indices: lazy map 

        new(array) -> Pattern
        checkEquality (Pattern) -> bool
*/
use std::collections::HashMap;
use super::lazy_hashmap::LazyHashMap;
pub use self::pattern::*;

pub struct Pattern <const N: usize, F> 
    where F: Fn() -> HashMap<String, i32> 
{
    pattern: [String;N],
    size: usize,
    indices: LazyHashMap<String, i32, F>,
}

impl <const N: usize, F> Pattern <N,F> 
    where F: Fn() -> HashMap<String, i32> 
    {

    pub fn new (array: [String;N]) -> Self {
        // let func: Fn() -> HashMap<T, i32> = 
        Self{
            pattern: array,
            size: N,
            indices: LazyHashMap::new::<Fn() -> HashMap<String, i32>>(||{
                let mut map = HashMap::new();
                for i in 0..array.iter().len() {
                    map.insert(array[i], i);
                }
                map
            }),
        }
    }

    fn checkEquality (self, other: Pattern<N,F>) -> bool {
        for i in 0..self.pattern.iter().len() {
                if self.pattern[i] != other.pattern[i]{
                    return false;
                }
        }
        true
    }    

}


