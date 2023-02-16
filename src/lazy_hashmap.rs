use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

pub struct LazyHashMap<K, V, F>
    where F: Fn() -> HashMap<K, V>
{
    data: Rc<RefCell<Option<HashMap<K, V>>>>,
    populate_fn: F,
}

impl<K, V, F> LazyHashMap<K, V, F>
    where F: Fn() -> HashMap<K, V>
{
    pub fn new(populate_fn: F) -> Self {
        Self {
            data: Rc::new(RefCell::new(None)),
            populate_fn,
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        let mut data = self.data.borrow_mut();

        if data.is_none() {
            *data = Some((self.populate_fn)());
        }

        data.as_ref().unwrap().get(key)
    }
}

/*
    let mut lazy_map = LazyHashMap::new(|| {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map
});

let value = lazy_map.get("key1").unwrap();
println!("Value: {}", value);
*/