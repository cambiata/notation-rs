pub use crate::beamgroup::*;
pub use crate::calc::*;
pub use crate::complex::*;
pub use crate::core::*;
pub use crate::head::*;
pub use crate::note::*;
pub use crate::part::*;
pub use crate::qcode::*;
pub use crate::render::*;
pub use crate::render_items::*;
pub use crate::testdata::*;
pub use crate::voice::*;

// use serde::{Deserialize, Serialize};

pub use std::fmt;
pub use std::{cell::RefCell, fmt::Debug, rc::Rc};
pub use std::{sync::atomic::AtomicUsize, sync::atomic::Ordering};

pub use crate::error::NotationError::*;
pub type Result<T> = anyhow::Result<T>;

// global id
pub static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
