#![warn(missing_docs)]
//! `my_library` provides a suite of helpers to create games with Bevy.(1)
//! 
//! ## What's Included?(2)
//! 
//! `my_library` includes:(3)
//! 
//! * Random number generation facilities.(4)
//! 
//! ## Feature Flags
//! 
//! The following feature flags are supported.
//! 
//! ### Random Number Generation
//! 
//! * The `locking` feature enables interior mutability inside 
//! [`RandomNumberGenerator`], (5)
//!   allowing it to be used as a resource (`Res<RandomNumberGenerator`) 
//! rather than requiring mutability (`ResMut<RandomNumberGenerator>`)
//! * You can control which random number generation algorithm is used by 
//! specifying *one* of:
//!    * `xorshift` to use the XorShift algorithm.
//!    * `pcg` to use the PCG algorithm.

#[cfg(not(feature = "locking"))]
mod random;
#[cfg(not(feature = "locking"))]
pub use random::*;

#[cfg(feature = "locking")]
mod random_locking;
#[cfg(feature = "locking")]
pub use random_locking::*;

/// [`RandomNumberGenerator`] wraps the `rand` crate. The `rand` crate
/// is re-exported for your convenience.
pub mod rand {
    pub use rand::*;
}
