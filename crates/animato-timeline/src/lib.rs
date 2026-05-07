//! # animato-timeline
//!
//! Timeline composition for Animato animations.
//!
//! This crate provides [`Timeline`] for concurrent composition, [`Sequence`] for
//! chained animation steps, and [`stagger()`] for offsetting many animations by a
//! fixed delay.
//!
//! ## Quick Start
//!
//! ```rust
//! use animato_core::{Easing, Update};
//! use animato_timeline::{At, Timeline};
//! use animato_tween::Tween;
//!
//! let fade = Tween::new(0.0_f32, 1.0)
//!     .duration(1.0)
//!     .easing(Easing::EaseOutCubic)
//!     .build();
//!
//! let slide = Tween::new(0.0_f32, 100.0).duration(0.5).build();
//!
//! let mut timeline = Timeline::new()
//!     .add("fade", fade, At::Start)
//!     .add("slide", slide, At::Offset(0.25));
//!
//! timeline.play();
//! timeline.update(0.5);
//! assert!(timeline.progress() > 0.0);
//! ```
//!
//! ## Feature Flags
//!
//! | Feature | Effect |
//! |---------|--------|
//! | `std`   | Enables std-dependent features in core and tween, plus callbacks |
//! | `serde` | Derives `Serialize`/`Deserialize` on supported public enums |
//! | `tokio` | Enables [`Timeline::wait()`] completion futures |

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

extern crate alloc;

pub mod sequence;
pub mod stagger;
pub mod timeline;

pub use sequence::Sequence;
pub use stagger::stagger;
pub use timeline::{At, Timeline, TimelineState};
