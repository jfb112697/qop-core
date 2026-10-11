#![allow(dead_code)]
pub mod engine;
pub mod key;
pub mod temperament;

pub use engine::{Engine, NoteEvent, Player, Voice};
pub use key::{Analog, Key};
pub use temperament::Temperament;

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Edit;

#[derive(Debug, Clone)]
pub struct Play;

pub trait QopPart<T>: Debug + Clone + Default + Serialize + for<'a> Deserialize<'a> {
    fn handle_engine_output(&self, qop: (i32, f64)) -> T;

    fn output_to_engine(&self) -> (i32, f64);
}
