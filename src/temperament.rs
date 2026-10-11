pub mod intervals;
pub mod scale_list;

use crate::*;
use better_default::Default;
use duplicate::duplicate_item;
use nestify::nest;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};
use winit::keyboard::KeyCode;

nest! {
    #[repr(C)]*
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]*
    pub(crate) struct Temperament<Mode = Edit> {
        pub(crate) _mode: PhantomData<Mode>,
        pub name: String,
        pub description: String,
        temperament_type: 
            pub enum TemperamentType {
                EqualTemperament(f64, f64),
                PrimeLimit(usize),
                #[default]
                Arbitrary,
            },
        #[default(69usize)]
        pub(crate) reference_note: usize,
        #[default(440.0f64)]
        pub(crate) tuning_hz: f64,
        #[default(4i64)]
        octave_label: i64,
        #[default(2.0f64)]
        pub(crate) octave_scalar_factor: f64,
        #[default(["C", "C♯/D♭", "D", "D♯/E♭", "E", "F", "F♯/G♭", "G", "G♯/A♭", "A", "A♯/B♭", "B"].iter().map(|i: &&str| i.to_string()).collect::<Vec<String>>())]
        pub(crate) note_class_set: Vec<String>,
        #[default(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0])]
        pub(crate) intervals: Vec<f64>,
    }
}

impl Temperament<Edit> {
    pub fn to_play(&self) -> Temperament<Play> {
        Temperament {
            _mode: PhantomData,
            name: self.name.clone(),
            description: self.description.clone(),
            reference_note: self.reference_note.clone(),
            tuning_hz: self.tuning_hz.clone(),
            note_class_set: self.note_class_set.clone(),
            octave_scalar_factor: self.octave_scalar_factor.clone(),
            intervals: self.intervals.clone(),
            octave_label: self.octave_label.clone(),
            temperament_type: self.temperament_type.clone(),
        }
    }

    pub(crate) fn new(
        name: Option<String>,
        description: Option<String>,
        temperament_type: Option<TemperamentType>,
        reference_note: Option<usize>,
        tuning_hz: Option<f64>,
        octave_label: Option<i64>,
        octave_scalar_factor: Option<f64>,
        note_class_set: Option<Vec<String>>,
        intervals: Option<Vec<f64>>,
    ) -> Temperament<Edit> {
        let d: Temperament<Edit> = Temperament::default();
        let mut n: Temperament<Edit> = Temperament {
            _mode: PhantomData,
            name: name.unwrap_or(d.name),
            description: description.unwrap_or(d.description),
            temperament_type: temperament_type.unwrap_or(d.temperament_type),
            reference_note: reference_note.unwrap_or(d.reference_note),
            tuning_hz: tuning_hz.unwrap_or(d.tuning_hz),
            octave_label: octave_label.unwrap_or(d.octave_label),
            octave_scalar_factor: octave_scalar_factor.unwrap_or(d.octave_scalar_factor),
            note_class_set: note_class_set.unwrap_or(d.note_class_set),
            intervals: intervals.unwrap_or(d.intervals),
        };
        
        // based on what is present in the input, build the other params from that
        // check for list of parameters in order of precedence

        if let Some(interval_list) = n.temperament_type.intervals() {
            n.intervals = interval_list;
        }

        n
    }
}

impl TemperamentType {
    fn name(&self, f: f64) -> String {
        match self {
            TemperamentType::EqualTemperament(d, _o) => format!("{f} hz {d}-Tone Equal Temperament Scale"),
            TemperamentType::PrimeLimit(p) => format!("{f} hz {p}-Limit Intonation Scale"),
            TemperamentType::Arbitrary => format!("{f} hz Arbitrary Scale"), 
        }
    }

    fn intervals(&self) -> Option<Vec<f64>> {
        match self {
            TemperamentType::EqualTemperament(_d, _o) => todo!(),
            TemperamentType::PrimeLimit(_p) => todo!(),
            TemperamentType::Arbitrary => None,
        }
    }
}
