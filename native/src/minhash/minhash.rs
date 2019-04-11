use std::fmt::Debug;
use std::str;
use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;
use std::hash::Hasher;
use twox_hash::XxHash;

#[derive(Debug)]
pub struct MinHash {
    hash: Vec<u32>,
}

static MINHASH_PRIME: u32 = 4294967291;
static MAX_HASH: u32 = 4294967295;
static XXHASH_SEED: u64 = 0xDEADBEEF;

fn hash_str(input: &String) -> u32 {
    let mut hash: u32 = 0;

    for c in input.chars() {
        hash = (hash << 5) - hash + (c as u32);
    }

    hash
}

impl MinHash {
    pub fn new(num_perm: usize) -> MinHash {
        let mut buffer: Vec<u32> = Vec::with_capacity(num_perm);

        let mut i = 0;

        while i < num_perm {
            buffer[i] = MAX_HASH;
            i += 1;
        }

        MinHash { hash: buffer }
    }

    pub fn num_perm(&self) -> usize {
        self.hash.capacity()
    }

    pub fn update(&mut self, input: String) {
        let mut index = 0;
        let mut num_perm = self.num_perm();

        while index < num_perm {
            let a = 5;//permA[index];
            let b = 6;//permB[index];
            let hash = (a * hash_str(&input) + b) % MINHASH_PRIME;

            if hash < self.hash[index] {
                self.hash[index] = hash;
            }

            index += 1;
        }
    }

    pub fn jaccard(&self, h: MinHash) -> f64 {
        let mut shared: u32 = 0;

        for (i, v) in self.hash.iter().enumerate() {
            if h.hash[i] == *v {
                shared += 1;
            }
        }

        (shared as f64) / (self.num_perm() as f64)
    }
}

#[derive(Debug)]
pub struct LshIndex {
    index: HashMap<u64, HashSet<String>>,
    band_size: u32,
}

fn get_hash_bands(hashes: Vec<u32>, band_size: u32) -> Vec<u64> {
    let mut ret: Vec<u64> = Vec::new();
    let size = hashes.capacity();

    let mut i: usize = 0;
    let _band_size = (band_size as usize);

    while i < size {
        let end = i + _band_size;

        let mut hasher = XxHash::with_seed(XXHASH_SEED);

        while i < end {
            hasher.write_u32(hashes[i]);
            i += 1;
        }

        ret.push(hasher.finish());
    }

    ret
}

impl LshIndex {
    pub fn new(band_size: u32) -> LshIndex {
        LshIndex {
            index: HashMap::new(),
            band_size: band_size,
        }
    }

    pub fn insert(&mut self, key: String, h: MinHash) {
        let hash_bands = get_hash_bands(h.hash, self.band_size);

        for band in &hash_bands {
            self.index
                .entry(*band)
                .or_insert_with(HashSet::new)
                .insert(key.to_owned());
        }
    }

    pub fn query(&self, h: MinHash) -> Vec<String> {
        let hash_bands = get_hash_bands(h.hash, self.band_size);
        let mut ret: HashSet<String> = HashSet::new();

        for band in &hash_bands {
            match self.index.get(band) {
                Some(set) => {
                    for val in set {
                        ret.insert(val.to_owned());
                    }
                }
                None => (),
            }
        }

        let mut vec: Vec<String> = Vec::with_capacity(ret.len());

        for v in &ret {
            vec.push(v.to_owned());
        }

        vec
    }
}