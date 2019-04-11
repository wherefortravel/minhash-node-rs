# minhash-node-rs

`minhash-node-rs` is a library for Node.js designed to calculate the MinHash of inputs, as well as being able to index them and allow sub-linear query time in a LSH index.

It is written in Rust for high performance and (relatively) low memory overhead, as well as memory safety.

## Why?

Why use MinHash? Sometimes you might want to look for documents that are *almost* the same, but not exactly, and you want to find them in sub-linear time (less than O(N) on average). `LshIndex` provides a way to look up near duplicates for a document in near constant time.

Why use Rust? Node.js & V8 can have very poor performance with the garbage collector when objects get large. Since objects such as the LSH index can become huge, we don't want to tax the GC too much. Writing this natively allows for that.

## How do I use it?

Here is an example:

```javascript
import { PermGen, MinHash, LshIndex } from 'minhash-node-rs'

const permGen = new PermGen(128) // 128 is the number of permutations to use when hashing

const hash = new MinHash(permGen) // we allocate MinHash using the permutations of previous PermGen instance

hash.update(permGen, 'hello') // add the word hello
hash.update(permGen, 'world') // add the word world

const hash2 = new MinHash(permGen)

hash2.update(permGen, 'world') // add words
hash2.update(permGen, 'test') // add words

console.log(hash.jaccard(hash2)) // print jaccard similarity

const index = new LshIndex() // construct LSH index

index.insert('hash', hash) // insert hash

console.log(index.query(hash2)) // query for near duplicates

```

## Who uses this?

[WhereTo.com](https://whereto.com) uses this in production to detect near duplicate data. If you use this in production, feel free to submit a Pull Request.
