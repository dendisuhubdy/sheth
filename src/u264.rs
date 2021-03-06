use arrayref::array_refs;
use bigint::U256;

#[cfg(feature = "std")]
use std::cmp::{Eq, Ordering};
#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
use std::hash::{Hash, Hasher};
#[cfg(feature = "std")]
use std::ops::{Add, BitAnd, Not, Shl, Shr, Sub};

#[cfg(not(feature = "std"))]
use core::cmp::{Eq, Ordering};
#[cfg(not(feature = "std"))]
use core::fmt;
#[cfg(not(feature = "std"))]
use core::hash::{Hash, Hasher};
#[cfg(not(feature = "std"))]
use core::ops::{Add, BitAnd, Not, Shl, Shr, Sub};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct U264([u8; 33]);

impl U264 {
    pub fn zero() -> Self {
        Self([0; 33])
    }

    pub fn one() -> Self {
        Self([
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ])
    }

    pub fn overflowing_add(self, other: Self) -> (Self, bool) {
        let Self(ref me) = self;
        let Self(ref you) = other;

        let mut ret = [0u8; 33];
        let mut carry;

        // TODO: Write macro to unroll all this:

        let (me_1, me_2, me_3, me_4, me_5) = array_refs!(me, 8, 8, 8, 8, 1);
        let (you_1, you_2, you_3, you_4, you_5) = array_refs!(you, 8, 8, 8, 8, 1);

        // Convert from le bytes to unsigned integers
        let me_1 = u64::from_le_bytes(*me_1);
        let me_2 = u64::from_le_bytes(*me_2);
        let me_3 = u64::from_le_bytes(*me_3);
        let me_4 = u64::from_le_bytes(*me_4);
        let me_5 = u8::from_le_bytes(*me_5);

        let you_1 = u64::from_le_bytes(*you_1);
        let you_2 = u64::from_le_bytes(*you_2);
        let you_3 = u64::from_le_bytes(*you_3);
        let you_4 = u64::from_le_bytes(*you_4);
        let you_5 = u8::from_le_bytes(*you_5);

        // Self[0..8]
        let (v, o1) = me_1.overflowing_add(you_1);
        ret[0..8].copy_from_slice(&v.to_le_bytes());
        carry = o1;

        // Self[8..16]
        let (v, o1) = me_2.overflowing_add(you_2);
        let (v, o2) = v.overflowing_add(if carry { 1 } else { 0 });
        ret[8..16].copy_from_slice(&v.to_le_bytes());
        carry = o1 || o2;

        // Self[16..24]
        let (v, o1) = me_3.overflowing_add(you_3);
        let (v, o2) = v.overflowing_add(if carry { 1 } else { 0 });
        ret[16..24].copy_from_slice(&v.to_le_bytes());
        carry = o1 || o2;

        // Self[24..32]
        let (v, o1) = me_4.overflowing_add(you_4);
        let (v, o2) = v.overflowing_add(if carry { 1 } else { 0 });
        ret[24..32].copy_from_slice(&v.to_le_bytes());
        carry = o1 || o2;

        // Self[33]
        let (v, o1) = me_5.overflowing_add(you_5);
        let (v, o2) = v.overflowing_add(if carry { 1 } else { 0 });
        ret[32] = v;
        carry = o1 || o2;

        (U264(ret), carry)
    }

    pub fn overflowing_sub(self, other: Self) -> (Self, bool) {
        let Self(ref me) = self;
        let Self(ref you) = other;

        let mut ret = [0u8; 33];
        let mut carry;

        // TODO: Write macro to unroll all this:

        let (me_1, me_2, me_3, me_4, me_5) = array_refs!(me, 8, 8, 8, 8, 1);
        let (you_1, you_2, you_3, you_4, you_5) = array_refs!(you, 8, 8, 8, 8, 1);

        // Convert from le bytes to unsigned integers
        let me_1 = u64::from_le_bytes(*me_1);
        let me_2 = u64::from_le_bytes(*me_2);
        let me_3 = u64::from_le_bytes(*me_3);
        let me_4 = u64::from_le_bytes(*me_4);
        let me_5 = u8::from_le_bytes(*me_5);

        let you_1 = u64::from_le_bytes(*you_1);
        let you_2 = u64::from_le_bytes(*you_2);
        let you_3 = u64::from_le_bytes(*you_3);
        let you_4 = u64::from_le_bytes(*you_4);
        let you_5 = u8::from_le_bytes(*you_5);

        // Self[0..8]
        let (v, o1) = me_1.overflowing_sub(you_1);
        ret[0..8].copy_from_slice(&v.to_le_bytes());
        carry = o1;

        // Self[8..16]
        let (v, o1) = me_2.overflowing_sub(you_2);
        let (v, o2) = v.overflowing_sub(if carry { 1 } else { 0 });
        ret[8..16].copy_from_slice(&v.to_le_bytes());
        carry = o1 || o2;

        // Self[16..24]
        let (v, o1) = me_3.overflowing_sub(you_3);
        let (v, o2) = v.overflowing_sub(if carry { 1 } else { 0 });
        ret[16..24].copy_from_slice(&v.to_le_bytes());
        carry = o1 || o2;

        // Self[24..32]
        let (v, o1) = me_4.overflowing_sub(you_4);
        let (v, o2) = v.overflowing_sub(if carry { 1 } else { 0 });
        ret[24..32].copy_from_slice(&v.to_le_bytes());
        carry = o1 || o2;

        // Self[33]
        let (v, o1) = me_5.overflowing_sub(you_5);
        let (v, o2) = v.overflowing_sub(if carry { 1 } else { 0 });
        ret[32] = v;
        carry = o1 || o2;

        (U264(ret), carry)
    }

    pub fn low_u32(&self) -> u32 {
        let &Self(ref arr) = self;
        let (arr, _) = array_refs!(arr, 4, 29);
        u32::from_le_bytes(*arr)
    }

    pub fn as_le_bytes(&self) -> &[u8; 33] {
        let &U264(ref me) = self;
        me
    }
}

impl Add for U264 {
    type Output = U264;

    fn add(self, other: U264) -> U264 {
        let (o, v) = self.overflowing_add(other);
        assert!(v == false);
        o
    }
}

impl Sub for U264 {
    type Output = U264;

    fn sub(self, other: U264) -> U264 {
        let (o, v) = self.overflowing_sub(other);
        assert!(v == false);
        o
    }
}

impl Shr<usize> for U264 {
    type Output = U264;

    fn shr(self, shift: usize) -> U264 {
        let U264(ref original) = self;
        let mut ret = [0u8; 33];

        let word_shift = shift / 8;
        let bit_shift = shift % 8;

        for i in word_shift..33 {
            // Shift
            ret[i - word_shift] += original[i] >> bit_shift;
            // Carry
            if bit_shift > 0 && i < 33 - 1 {
                ret[i - word_shift] += original[i + 1] << (8 - bit_shift);
            }
        }

        U264(ret)
    }
}

impl Shl<usize> for U264 {
    type Output = U264;

    fn shl(self, shift: usize) -> U264 {
        let U264(ref original) = self;
        let mut ret = [0u8; 33];

        let word_shift = shift / 8;
        let bit_shift = shift % 8;

        for i in 0..33 {
            // Shift
            if bit_shift < 8 && i + word_shift < 33 {
                ret[i + word_shift] += original[i] << bit_shift;
            }
            // Carry
            if bit_shift > 0 && i + word_shift + 1 < 33 {
                ret[i + word_shift + 1] += original[i] >> (8 - bit_shift);
            }
        }

        U264(ret)
    }
}

impl BitAnd for U264 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let Self(ref me) = self;
        let Self(ref you) = rhs;

        let (me_1, me_2, me_3, me_4, me_5) = array_refs!(me, 8, 8, 8, 8, 1);
        let (you_1, you_2, you_3, you_4, you_5) = array_refs!(you, 8, 8, 8, 8, 1);

        let mut ret = [0u8; 33];
        ret[0..8].copy_from_slice(
            &(u64::from_le_bytes(*me_1) & u64::from_le_bytes(*you_1)).to_le_bytes(),
        );
        ret[8..16].copy_from_slice(
            &(u64::from_le_bytes(*me_2) & u64::from_le_bytes(*you_2)).to_le_bytes(),
        );
        ret[16..24].copy_from_slice(
            &(u64::from_le_bytes(*me_3) & u64::from_le_bytes(*you_3)).to_le_bytes(),
        );
        ret[24..32].copy_from_slice(
            &(u64::from_le_bytes(*me_4) & u64::from_le_bytes(*you_4)).to_le_bytes(),
        );
        ret[32..33]
            .copy_from_slice(&(u8::from_le_bytes(*me_5) & u8::from_le_bytes(*you_5)).to_le_bytes());

        ret.into()
    }
}

impl Not for U264 {
    type Output = Self;

    fn not(self) -> Self::Output {
        let Self(ref me) = self;

        let (me_1, me_2, me_3, me_4, me_5) = array_refs!(me, 8, 8, 8, 8, 1);

        let mut ret = [0u8; 33];
        ret[0..8].copy_from_slice(&(!u64::from_le_bytes(*me_1)).to_le_bytes());
        ret[8..16].copy_from_slice(&(!u64::from_le_bytes(*me_2)).to_le_bytes());
        ret[16..24].copy_from_slice(&(!u64::from_le_bytes(*me_3)).to_le_bytes());
        ret[24..32].copy_from_slice(&(!u64::from_le_bytes(*me_4)).to_le_bytes());
        ret[32..33].copy_from_slice(&(!u8::from_le_bytes(*me_5)).to_le_bytes());

        ret.into()
    }
}

impl Eq for U264 {}

impl PartialEq for U264 {
    fn eq(&self, other: &U264) -> bool {
        let U264(ref me) = self;
        let U264(ref you) = other;

        // TODO: Write macro to unroll all this:

        let (me_1, me_2, me_3, me_4, me_5) = array_refs!(me, 8, 8, 8, 8, 1);
        let (you_1, you_2, you_3, you_4, you_5) = array_refs!(you, 8, 8, 8, 8, 1);

        // Convert from le bytes to unsigned integers
        let me_1 = u64::from_le_bytes(*me_1);
        let me_2 = u64::from_le_bytes(*me_2);
        let me_3 = u64::from_le_bytes(*me_3);
        let me_4 = u64::from_le_bytes(*me_4);
        let me_5 = u8::from_le_bytes(*me_5);
        let you_1 = u64::from_le_bytes(*you_1);
        let you_2 = u64::from_le_bytes(*you_2);
        let you_3 = u64::from_le_bytes(*you_3);
        let you_4 = u64::from_le_bytes(*you_4);
        let you_5 = u8::from_le_bytes(*you_5);

        // These comparisions need to be in this otherwise would produce incorrect results

        if me_1 < you_1 {
            return false;
        }

        if me_1 > you_1 {
            return false;
        }

        if me_2 < you_2 {
            return false;
        }

        if me_2 > you_2 {
            return false;
        }

        if me_3 < you_3 {
            return false;
        }

        if me_3 > you_3 {
            return false;
        }

        if me_4 < you_4 {
            return false;
        }

        if me_4 > you_4 {
            return false;
        }

        if me_5 < you_5 {
            return false;
        }

        if me_5 > you_5 {
            return false;
        }

        true
    }
}

impl PartialOrd for U264 {
    fn partial_cmp(&self, other: &U264) -> Option<Ordering> {
        let Self(ref me) = self;
        let Self(ref you) = other;

        Some(me.cmp(you))
    }
}

impl Ord for U264 {
    fn cmp(&self, other: &Self) -> Ordering {
        let Self(ref me) = self;
        let Self(ref you) = other;
        me.cmp(you)
    }
}

impl Hash for U264 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Self(ref me) = self;
        me.hash(state);
    }
}

// Chops off most significant byte
impl From<U256> for U264 {
    fn from(n: U256) -> U264 {
        let mut buf = [0u8; 33];
        n.to_little_endian(&mut buf[0..32]);
        U264::from(buf)
    }
}

impl From<u8> for U264 {
    fn from(n: u8) -> U264 {
        U264([
            n, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ])
    }
}

impl From<[u8; 33]> for U264 {
    fn from(arr: [u8; 33]) -> U264 {
        U264(arr)
    }
}

impl From<U264> for [u8; 33] {
    fn from(n: U264) -> [u8; 33] {
        n.0
    }
}

impl fmt::Debug for U264 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self.0[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let x = U264::zero();
        let y = U264::one();
        assert_eq!(x + y, U264::one());
    }

    #[test]
    fn overflowing_add() {
        let x = U264::one();
        let y = U264([
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ]);
        assert_eq!(x.overflowing_add(U264::zero()), (x, false));
        assert_eq!(x.overflowing_add(y), (U264::zero(), true));
    }

    #[test]
    fn shift_right_zero() {
        let x = U264::zero();
        let y = x.shr(10);
        assert_eq!(x, y);
    }

    #[test]
    fn shift_right() {
        let x = U264::one();
        assert_eq!(x.shr(1), U264::zero());
    }

    #[test]
    fn shift_left_zero() {
        let x = U264::zero();
        let y = x.shl(10);
        assert_eq!(x, y);
    }

    #[test]
    fn shift_left() {
        let x = U264::one();
        assert_eq!(x.shl(1), U264::from(2));
    }

}
