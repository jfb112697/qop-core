#![allow(dead_code)]
pub mod engine;
pub mod key;
pub mod temperament;

pub use engine::Engine;
pub use key::Key;
pub use temperament::Temperament;

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Edit;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Play;

pub trait QopPart<T>: Debug + Clone + Default + Serialize + for<'a> Deserialize<'a> {
    fn handle_engine_output(&self, qop: (usize, f64)) -> T;

    fn output_to_engine(&self) -> (usize, f64);
}
