//! Deterministic primitives for traditional calendrical and correlative-cosmology systems.
//!
//! Public cycle order is part of the crate contract: Heavenly Stems and Earthly
//! Branches use their canonical order, and downstream index arithmetic assumes
//! those orders remain stable.

pub mod gan;
pub mod ganzhi;
pub mod traits;
pub mod wuxing;
pub mod yinyang;
pub mod zhi;

#[cfg(test)]
mod tests;

pub use gan::*;
pub use ganzhi::*;
pub use traits::*;
pub use wuxing::*;
pub use yinyang::*;
pub use zhi::*;
