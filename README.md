# Bloom Filter
Bloom filter is a data structure that can check the element whether exists in a certain collection or not. But It probably checks in error and we call those false positive matches. More generally, average fewer than 10 bits per element are required for a 1% false positive probability, independent of the size or number of elements in the set.

## Usage
```rust
let mut bf = BloomFilter::new(10);
let b1 = Bytes::from(&b"hello"[..]);
let b2 = Bytes::from(&b"world"[..]);

bf.add(&b1);
let filter = bf.generate();
println!("{}", bf.contains(&filter, &b1)); // true
println!("{}", bf.contains(&filter, &b2)); // false
```

## References
- [Network Applications of Bloom Filters: A Survey](http://www.eecs.harvard.edu/~michaelm/postscripts/im2005b.pdf)
- [An Improved Construction for Counting Bloom Filters](http://theory.stanford.edu/~rinap/papers/esa2006b.pdf)
- [goleveldb/leveldb/filter/bloom.go](https://github.com/syndtr/goleveldb/blob/master/leveldb/filter/bloom.go)
