
extern crate bytes;

use bytes::{BufMut, BytesMut, Bytes, Buf};

pub mod codec {
    pub fn get_u32_little_end(arr: &[u8]) -> u32 {
        ((arr[0] as u32) <<  0) +
            ((arr[1] as u32) <<  8) +
            ((arr[2] as u32) << 16) +
            ((arr[3] as u32) << 24)
    }
}

fn hash(data: &Bytes, seed: u32) -> u32 {
    let m = 0xc6a4a793 as u32;
    let r = 24 as u32;
    let mut h = seed ^ (data.len() as u64 * m as u64) as u32;
    let n = data.len() - data.len() % 4;
    let mut i = 0;

    while i < n {
        h += codec::get_u32_little_end(&data[i..]) as u32;
        h = (h as u64 * m as u64) as u32;
        h ^= (h >> 16);
        i += 4;
    }

    let flag = data.len() - i;
    if flag == 3 {
        h += (data.len() as u32) << 16;
    } else if flag == 2 {
        h += (data.len() as u32) << 8;
    } else if flag == 1 {
        h += data[i] as u32;
        h = (h as u64 * m as u64) as u32;
        h ^= (h >> r);
    }

    return h
}

fn bloom_hash(data: &Bytes) -> u32 {
    hash(data, 0xbc9f1d34)
}

pub struct BloomFilter {
    bits_per_key: usize,
    k: u8,
    key_hashes: Vec<u32>,
}

impl BloomFilter {

    pub fn new(bits_per_key: usize) -> Self {
        let mut k = (bits_per_key as f64 * 0.69) as u8;
        if k < 1 { k = 1; }
        if k > 30 { k = 30; }
        BloomFilter { bits_per_key, k, key_hashes: vec![] }
    }

    pub fn contains(&self, filter: &Bytes, key: &Bytes) -> bool {
        let n_bytes = filter.len() - 1;
        if n_bytes < 1 {
            return false
        }
        let n_bits = (n_bytes * 8) as u32;

        let k = filter[n_bytes];
        if k > 30 {
            return true
        }

        let mut kh = bloom_hash(key);
        let delta = (kh >> 17) | (kh << 15);
        for _ in 0..k {
            let bitpos = (kh % n_bits) as usize;
            if filter[bitpos/8] as u32 & (1 << (bitpos % 8)) == 0 {
                return false
            }
            kh = (kh as u64 + delta as u64) as u32;
        }

        return true
    }

    pub fn add(&mut self, key: &Bytes) {
        self.key_hashes.push(bloom_hash(key))
    }

    pub fn generate(&mut self) -> Bytes {
        let mut n_bits = (self.key_hashes.len() * self.bits_per_key) as u32;
        if n_bits < 64 {
            n_bits = 64;
        }
        let n_bytes = (n_bits + 7) / 8;
        n_bits = n_bytes * 8;

        let mut dest = BytesMut::new();
        dest.resize(n_bytes as usize + 1, 0);
        dest[n_bytes as usize] = self.k;
        for v in &self.key_hashes {
            let mut kh = v.clone();
            let delta = (kh >> 17) | (kh << 15);
            for _ in 0..self.k {
                let bitpos = (kh % n_bits) as usize;
                dest[bitpos/8] |= (1 << (bitpos % 8));
                kh = (kh as u64 + delta as u64) as u32;
            }
        }
        self.key_hashes.clear();

        dest.freeze()
    }

}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut, BufMut, Buf};
    use crate::BloomFilter;

    fn num_to_bytes(num: u32) -> Bytes {
        let mut bs = BytesMut::new();
        bs.put_u32_le(num);
        bs.freeze()
    }

    #[test]
    fn it_works() {
        let mut bf = BloomFilter::new(10);
        let n = 10000;
        for i in 0..n {
            bf.add(&num_to_bytes(i))
        }
        let filter = bf.generate();

        for i in 0..n {
            if !bf.contains(&filter, &num_to_bytes(i)) {
                panic!(format!("Error in {}", i))
            }
        }

        let mut rate: f32 = 0.0;
        for i in 0..n {
            if bf.contains(&filter, &num_to_bytes(i + n + 1)) {
                rate += 1.0;
            }
        }

        rate /= n as f32;
        if rate > 0.02 {
            panic!(format!("False positive rate is more than 2%%, got {}, at len {}", rate, n))
        } else {
            println!("False positive rate is {}", rate)
        }
    }
}
