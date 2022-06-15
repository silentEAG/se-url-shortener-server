

#[inline]
fn rotl32(x: u32, r: i8) -> u32 {
    (x << r) | (x >> (32 - r))
}

#[inline]
fn _rotl64(x: u64, r: i8) -> u64 {
    (x << r) | (x >> (64 - r))
}

#[inline]
fn fmix32(h: u32) -> u32 {
    let mut res = h ^ (h >> 16);
    res = res.wrapping_mul(0x85ebca6b);
    res ^= res >> 13;
    res = res.wrapping_mul(0xc2b2ae35);
    res ^= res >> 16;
    res
}

#[inline]
fn _fmix64(h: u64) -> u64 {
    let mut res = h ^ (h >> 33);
    res *= 0xff51afd7ed558ccd_u64;
    res ^= res >> 33;
    res *= 0xc4ceb9fe1a85ec53_u64;
    res ^= res >> 33;
    res
}

pub fn murmur_hash3_x86_32(key: *const u8, len: usize, seed: u32) -> u32 {
    let data = key;
    let nblocks: usize = len / 4;
    
    let mut h1: u32 = seed;
  
    let c1: u32 = 0xcc9e2d51;
    let c2: u32 = 0x1b873593;
  
    let blocks: *const u32 = ((data as usize) + (nblocks * 4)) as *const u32;

    let mut block_data = data as *const u32;

    while block_data != blocks {
        unsafe {
            let mut k1: u32 = *block_data;
            
            k1 = k1.wrapping_mul(c1);
            k1 = rotl32(k1,15);
            k1 = k1.wrapping_mul(c2);
            
            h1 ^= k1;
            h1 = rotl32(h1,13); 
            h1 = ((h1 as u128) * 5 + 0xe6546b64) as u32;        
        }
        block_data = ((block_data as usize) + 4) as *const u32;
    }

    let tail: *const u8 = ((data as usize) + (nblocks * 4)) as *const u8;
    let mut k1: u32 = 0;
    let remain_len = len & 3;
    unsafe {
        if remain_len >= 3 {
            k1 ^= (*(((tail as usize) + 2) as *const u8) as u32) << 16;
        }
        if remain_len >= 2 {
            k1 ^= (*(((tail as usize) + 1) as *const u8) as u32) << 8;
        }
        if remain_len >= 1 {
            k1 ^= *tail as u32;
        }
    }
    k1 = k1.wrapping_mul(c1);
    k1 = rotl32(k1, 15);
    k1 = k1.wrapping_mul(c2);
    h1 ^= k1;

    h1 ^= len as u32;
    h1 = fmix32(h1);
    // println!("{:?}", h1);
    h1
}

#[test]
fn test_for_murmur_hash3_x86_32() {

    // come from https://github.com/PeterScott/murmur3/blob/master/test.c
    let s1 = "Hello, world!";
    let p1 = s1.as_ptr();
    assert_eq!(murmur_hash3_x86_32(p1, s1.len(), 1234), 4210478515);

    // self test
    let s2 = "SilentEEA has no money!";
    let p2 = s2.as_ptr();
    assert_eq!(murmur_hash3_x86_32(p2, s2.len(), 114514), 3148959311);
}