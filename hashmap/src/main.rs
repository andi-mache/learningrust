use std::collections::HashMap;
use std::hash::Hash;

use std::net;

//! a HashMap is a data structure that stores key-value pairs. It is a collection of items where
//! each item is stored as a key-value pair. The keys are unique, and they are used to retrieve the
//! values associated with them. HashMaps are useful when you need to store data that can be
//! accessed quickly using a key. just like a real-world dictionary, where you can look up the
//! meaning of a word by searching for it in the dictionary. In this case, the word is the key, and
//! the meaning is the value. HashMaps are also known as associative arrays, hash tables, or hash
//! maps. HashMaps are widely used in programming because they provide a fast way to store and look
//! up data. HashMaps are implemented using a data structure called a hash table. A hash table is a
//! data structure that stores key-value pairs. It uses a hash function to compute an index into an
//! array of buckets or slots, from which the desired value can be found. The hash function takes
//! the key as input and returns an integer that represents the index of the bucket where the value
//! is stored. The hash function should be fast and should distribute the keys uniformly across the
//! buckets to avoid collisions. A collision occurs when two keys hash to the same index. In this
//! case, the hash table needs to resolve the collision by using a collision resolution strategy.
//! There are several collision resolution strategies, such as chaining, open addressing, and
//! double hashing. HashMaps use chaining to resolve collisions. In chaining, each bucket in the
//! hash table is a linked list of key-value pairs. When a collision occurs, the key-value pair is
//! added to the linked list at the corresponding bucket. To retrieve a value from the hash table,
//! the hash function is applied to the key to compute the index of the bucket where the value is
//! stored. The linked list at that bucket is then searched for the key, and the corresponding
//! value is returned. HashMaps have an average time complexity of O(1) for insertion, deletion,
//! and lookup operations. However, in the worst case, the time complexity can be O(n) if all keys
//! hash to the same index, resulting in a long linked list. To avoid this worst-case scenario, the
//! hash function should be well-designed to distribute the keys uniformly across the buckets.
//! HashMaps are widely used in programming because they provide a fast way to store and look up
//! data. They are used in various applications, such as databases, compilers, interpreters, and
//! web servers. In Rust, the HashMap type is provided by the standard library. It is a generic
//! type that can store key-value pairs of any type. To use a HashMap, you need to import it from
//! the std::collections module. You can create a new HashMap using the new method, insert
//! key-value pairs using the insert method, and retrieve values using the get method. HashMaps are
//! mutable, which means you can modify them after they are created. You can add, remove, and
//! update key-value pairs in a HashMap. HashMaps are implemented using a hash table, which
//! provides fast access to key-value pairs. They are widely used in programming because they
//! provide a fast way to store and look up data. HashMaps are useful when you need to store data
//! that can be accessed quickly using a key. In this

/// Counter counts the number of times each value of type T has been seen.
/// k
struct Counter<T> {
    values: HashMap<T, u64>,
} 
impl<T: Eq + Hash> Counter<T> {
    /// Create a new Counter.
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }
    /// Count an occurrence of the given value.
    fn count(&mut self, value: T) {
        *self.values.entry(value).or_default() += 1;
    }
    /// Return the number of times the given value has been seen.
    fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}
fn main() {
    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);
    for i in 10..20 {
        println!("saw {} values equal to {}", ctr.times_seen(i), i);
    }
    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    println!("got {} apples", strctr.times_seen("apple"));
}
