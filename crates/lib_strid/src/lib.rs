// Copyright 2023 Natalie Baker // AGPLv3 //

mod smol_str;
pub use smol_str::*;

#[macro_export]
macro_rules! newtype_str_id {
    ($vis:vis $name:ident) => {
        
        #[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
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

        impl core::convert::From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::from_name(value)
            }
        }

    };
}

