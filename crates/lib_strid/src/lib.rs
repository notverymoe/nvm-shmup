// Copyright 2023 Natalie Baker // AGPLv3 //

mod smol_str;
pub use smol_str::*;

#[macro_export]
macro_rules! newtype_str_id {
    ($vis:vis $name:ident) => {
        
        #[derive(Debug, Clone)]
        #[repr(transparent)]
        $vis struct $name($crate::SmolStr);

        impl $name {

            pub const EMPTY: Self = Self($crate::SmolStr::EMPTY);

            $vis const fn from_name(id: &str) -> Self {
                Self($crate::SmolStr::new(id))
            }

            $vis const fn from_raw(id: u128) -> Self {
                Self($crate::SmolStr::from_raw(id))
            }

            $vis fn to_str(&self) -> String {
                self.0.to_str()
            }

            $vis fn to_raw(&self) -> u128 {
                self.0.to_raw()
            }
        }

        impl core::default::Default for $name {
            fn default() -> Self {
                Self(Default::default())
            }
        }

        unsafe impl core::marker::Send for $name { }
        unsafe impl core::marker::Sync for $name { }
        impl core::marker::Copy for $name { }
        impl core::cmp::Eq      for $name { }

        impl core::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl core::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        impl core::convert::From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::from_name(value)
            }
        }

    };
}

