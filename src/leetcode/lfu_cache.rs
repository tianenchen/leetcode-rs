use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, Clone)]
struct LFUCache {
    cache: std::collections::HashMap<i32, Rc<RefCell<LFU>>>,
    lfu: std::collections::BTreeSet<Rc<RefCell<LFU>>>,
    cap: i32,
    pos: i32,
    time: i32,
}

#[derive(PartialEq, Eq, Debug)]
struct LFU {
    cnt: i32,
    time: i32,
    key: i32,
    value: i32,
}

impl LFU {
    fn new(time: i32, key: i32, value: i32) -> Self {
        LFU {
            cnt: 1,
            time,
            key,
            value,
        }
    }
}

impl PartialOrd for LFU {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.cnt == other.cnt {
            Some(self.time.cmp(&other.time))
        } else {
            Some(self.cnt.cmp(&other.cnt))
        }
    }
}

impl Ord for LFU {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.cnt == other.cnt {
            self.time.cmp(&other.time)
        } else {
            self.cnt.cmp(&other.cnt)
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache {
            cache: std::collections::HashMap::new(),
            lfu: std::collections::BTreeSet::new(),
            cap: capacity,
            pos: 0,
            time: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(r) = self.cache.get_mut(&key) {
            self.time += 1;
            r.borrow_mut().time = self.time;
            r.borrow_mut().cnt += 1;
            r.borrow_mut().value.clone()
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.time += 1;
        if self.pos == self.cap {
            let lfu = self.lfu.iter().next().unwrap().clone();
            dbg!(lfu.clone());
            self.cache.remove(&lfu.borrow().key);
            let _ = self.lfu.remove(&lfu);
            let lfu = Rc::new(RefCell::new(LFU::new(self.time, key, value)));
            self.cache.insert(key, lfu.clone());
            self.lfu.insert(lfu);
        } else if let Some(v) = self.cache.get_mut(&key) {
            v.borrow_mut().value = value; //update new value
            v.borrow_mut().time = self.time;
            v.borrow_mut().cnt += 1;
            return;
        } else {
            self.pos += 1;
            let lfu = Rc::new(RefCell::new(LFU::new(self.time, key, value)));
            self.cache.insert(key, lfu.clone());
            self.lfu.insert(lfu);
        }
    }
}

#[test]
fn check() {
    // let mut cache = LFUCache::new(2);
    // dbg!(cache.clone());
    // cache.put(1, 1);
    // dbg!(cache.clone());
    // cache.put(2, 2);
    // dbg!(cache.clone());
    // cache.get(1);
    // dbg!(cache.clone());
    // cache.put(3, 3);
    // dbg!(cache.clone());
    // let res = cache.get(2);
    // assert_eq!(res,-1);

    // let mut cache = LFUCache::new(3);
    // dbg!(cache.clone());
    // cache.put(2,2);
    // dbg!(cache.clone());
    // cache.put(1,1);
    // dbg!(cache.clone());
    // cache.get(2);
    // dbg!(cache.clone());
    // cache.get(1);
    // dbg!(cache.clone());
    // cache.get(2);
    // dbg!(cache.clone());
    // cache.put(3,3);
    // dbg!(cache.clone());
    // cache.put(4,4);
    // dbg!(cache.clone());
    // let res = cache.get(3);
    // assert_eq!(res,-1);

    // unimplemented!()
}
