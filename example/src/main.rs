extern crate bytes;
use bytes::Bytes;
use bloom_filter::BloomFilter;

fn main() {
    let mut bf = BloomFilter::new(10);
    let b1 = Bytes::from(&b"hello"[..]);
    let b2 = Bytes::from(&b"world"[..]);

    bf.add(&b1);
    let filter = bf.generate();
    println!("{}", bf.contains(&filter, &b1)); // true
    println!("{}", bf.contains(&filter, &b2)); // false
}
