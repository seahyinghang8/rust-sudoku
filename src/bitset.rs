use std::fmt;

#[derive(Copy, Clone)]
pub struct BitSet {
    bits: i32
}

impl BitSet {
    pub fn new() -> BitSet {
        BitSet { bits: 0 }
    }

    pub fn contains(&self, value: u8) -> bool {
        (self.bits & (1 << value)) > 0
    }

    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    pub fn insert(&mut self, value: u8) {
        self.bits = self.bits | (1 << value);
    }

    pub fn remove(&mut self, value: u8) {
        self.bits = self.bits & !(1 << value);
    }

    pub fn flip(&mut self, mask: i32) {
        self.bits = !self.bits & mask;
    }

    pub fn extend(&mut self, set: &BitSet) {
        self.bits = self.bits | set.bits
    }

    pub fn len(&self) -> usize {
        let mut x = self.bits;
        x = (x & (0x55555555)) + ((x >> 1) & (0x55555555));
        x = (x & (0x33333333)) + ((x >> 2) & (0x33333333));
        x = (x & (0x0f0f0f0f)) + ((x >> 4) & (0x0f0f0f0f));
        x = (x & (0x00ff00ff)) + ((x >> 8) & (0x00ff00ff));
        x = (x & (0x0000ffff)) + ((x >> 16) & (0x0000ffff));
        return x as usize;
    }
}

impl fmt::Display for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "{{}}");
        }

        write!(f, "{{")?;
        let mut is_first = true;
        for i in 1..10 {
            if self.contains(i as u8) {
                if !is_first {
                    write!(f, ",")?;
                } else {
                    is_first = false;
                }
                write!(f, " {}", i)?;
            }
        }
        write!(f, " }}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_bitset() {
        let s = BitSet::new();
        assert!(s.is_empty());
    }

    #[test]
    fn insertion() {
        let mut s = BitSet::new();
        s.insert(1);
        assert!(s.contains(1));
        assert!(!s.contains(8));
        s.insert(3);
        assert!(s.contains(1));
        assert!(s.contains(3));
        assert!(!s.contains(2));
        assert!(!s.contains(6));
        assert!(!s.contains(9));
        s.insert(6);
        assert!(s.contains(1));
        assert!(s.contains(3));
        assert!(s.contains(6));
        assert!(!s.contains(2));
        assert!(!s.contains(9));
    }

    #[test]
    fn removal() {
        let mut s = BitSet::new();
        s.insert(1);
        s.insert(3);
        s.insert(6);
        s.insert(2);
        assert!(s.contains(1));
        assert!(s.contains(3));
        assert!(s.contains(6));
        assert!(s.contains(2));
        s.remove(3);
        assert!(s.contains(1));
        assert!(!s.contains(3));
        assert!(s.contains(6));
        assert!(s.contains(2));
        s.remove(2);
        assert!(s.contains(1));
        assert!(!s.contains(3));
        assert!(s.contains(6));
        assert!(!s.contains(2));
        assert!(!s.contains(4));
        s.remove(4);
        assert!(!s.contains(4));
        assert!(s.contains(1));
        assert!(!s.contains(3));
        assert!(s.contains(6));
        assert!(!s.contains(2));
        s.remove(1);
        s.remove(6);
        assert!(s.is_empty());
    }

    #[test]
    fn check_len() {
        let mut s = BitSet::new();
        assert_eq!(s.len(), 0);
        s.remove(3);
        assert_eq!(s.len(), 0);
        s.insert(1);
        assert_eq!(s.len(), 1);
        s.insert(1);
        s.insert(2);
        assert_eq!(s.len(), 2);
        s.insert(3);
        s.insert(5);
        s.insert(9);
        assert_eq!(s.len(), 5);
        s.remove(1);
        s.remove(2);
        s.remove(9);
        assert_eq!(s.len(), 2);
        s.remove(3);
        s.remove(5);
        assert_eq!(s.len(), 0);
    }

}