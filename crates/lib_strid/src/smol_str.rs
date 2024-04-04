// Copyright 2023 Natalie Baker // AGPLv3 //

use std::{fmt::{Display, Debug}, num::NonZeroU128};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SmolStr(NonZeroU128);

impl Default for SmolStr {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl SmolStr {

    pub const EMPTY: Self = Self(NonZeroU128::MIN);

    pub const fn new(str: &str) -> Self {
        match Self::try_new(str) {
            Ok(v)  => v,
            Err(e) => panic!("{}", e),
        }
    }

    pub const fn from_raw(value: u128) -> Self {
        // TODO verify correctness
        if let Some(value) = NonZeroU128::new((value << 1) | 1) {
            Self(value)
        } else {
            panic!("Failed to construct NonZeroU128 for result");
        }
    }

    pub const fn try_new(str: &str) -> Result<Self, &'static str> {
        if str.is_empty() {
            return Ok(Self::EMPTY);
        }

        if str.len() > 25 {
            return Err("String too long, max length 25");
        }

        let mut value:  u128  = 0;
        let mut offset: usize = 0;

        let mut i = 0; 
        loop {
            // // Loop Body // //
            let ch = str.as_bytes()[i];

            value |= (if ch >= b'a' && ch <= b'z' {
                1 + ch - b'a'
            } else if ch >= b'A' && ch <= b'Z' {
                1 + ch - b'A'
            } else if ch == b'_' {
                27
            } else {
                return Err("String contains invalid character, valid characters are alphabetic and underscore")
            } as u128) << offset;

            // // Loop Inc + Cond // //
            offset += 5;
            if offset > 120 { break; }

            i += 1;
            if i >= str.len() { break; }
        }

        Ok(Self::from_raw(value))
    }

    pub fn to_raw(self) -> u128 {
        self.0.get() >> 1
    }

    pub fn to_str(self) -> String {
        let mut result = String::new();
        let value = self.to_raw();
        for offset in (0..=120).step_by(5) {
            let ch = ((value >> offset) & 0b11111) as u8;
            if ch == 0 { break; }
            result.push((if ch < 27 { ch + b'A' - 1 } else { b'_' }).into());
        }
        result
    }

}

impl Debug for SmolStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SmolStr").field(&self.0).field(&self.to_str()).finish()
    }
}

impl Display for SmolStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_str())
    }
}

impl From<&str> for SmolStr {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod test {
    use crate::SmolStr;

    #[test]
    fn check_niche_opt() {
        assert_eq!(core::mem::size_of::<SmolStr>(), core::mem::size_of::<Option<SmolStr>>());
    }

    #[test]
    fn check_round_trip() {
        assert_eq!(SmolStr::new("HELLO").to_str(), "HELLO");
        assert_eq!(SmolStr::new("hello").to_str(), "HELLO");

        assert_eq!(SmolStr::new("HELLO_world").to_str(), "HELLO_WORLD");
        assert_eq!(SmolStr::new("hello_WORLD").to_str(), "HELLO_WORLD");
    }

}