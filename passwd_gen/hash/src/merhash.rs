// hash/src/merhash.rs
//! Mersenne Hash
//!
/// calculate hash value with mersenne prime 127
///
/// # Example
/// use hash::merhash::mersenne_hash;
///
/// let seed = "jdxjp";
/// let hash = mersenne_hash(&seed);
/// assert_eq!(2000375, hash);
pub fn mersenne_hash(seed: &str) -> usize {
    let mut hash: usize = 0;
    for (i, c) in seed.chars().enumerate() {
        hash += (i + 1) * (c as usize);
    }
    (hash % 127).pow(3) - 1
}
