use std::u16;
use std::ops::Add;
use std::collections::HashMap;
use std::convert::{Into, From};
use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct U15(u16);

const N: u16 = 1 << 15;
const N_SUB_1: U15 = U15(N - 1);
const ZERO_15: U15 = U15(0);
const ONE_15: U15 = U15(1);

impl fmt::Display for U15 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u16> for U15 {
    fn from(x: u16) -> U15 {
        U15(x % N)
    }
}

impl Add for U15 {
    type Output = U15;

    fn add(self, other: U15) -> U15 {
        U15(self.0.wrapping_add(other.0) % N)
    }
}

impl<'a, 'b> Add<&'b U15> for &'a U15 {
    type Output = U15;

    fn add(self, other: &'b U15) -> U15 {
        U15(self.0.wrapping_add(other.0) % N)
    }
}

impl<'a> Add<&'a U15> for U15 {
    type Output = U15;

    fn add(self, other: &'a U15) -> U15 {
        U15(self.0.wrapping_add(other.0) % N)
    }
}

impl<'a> Add<U15> for &'a U15 {
    type Output = U15;

    fn add(self, other: U15) -> U15 {
        U15(self.0.wrapping_add(other.0) % N)
    }
}

struct AckermannCache {
    r7: U15,
    cache: HashMap<(U15, U15), U15>,
}

impl AckermannCache {
    fn new<T: Into<U15>>(r7: T) -> AckermannCache {
        AckermannCache {
            r7: r7.into(),
            cache: HashMap::new(),
        }
    }

    fn get<T: Into<U15>>(&mut self, m_: T, n_: T) -> U15 {
        let m = m_.into();
        let n = n_.into();

        if !self.cache.contains_key(&(m, n)) {
            let val = if m == ZERO_15 {
                n + ONE_15
            } else if n == ZERO_15 {
                let r7 = self.r7;
                self.get(m + N_SUB_1, r7)
            } else {
                let new_n = self.get(m, n + N_SUB_1);
                self.get(m + N_SUB_1, new_n)
            };
            self.cache.insert((m, n), val);
        }

        self.cache[&(m, n)]
    }
}

fn main() {    
    for i in 1..N {
        let r7 = U15(i);
        let mut ac = AckermannCache::new(r7);
        let r0 = ac.get(4, 1);
        
        if r0 == 6.into() {
            println!("r7 = {: >5}, r0 = {: >5}", r7, r0);
        }
    }

}
