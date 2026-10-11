pub mod intervals;
pub mod scale_list;

use crate::*;
use better_default::Default;
use nestify::nest;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};

nest! {
    #[repr(C)]*
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]*
    pub struct Temperament<Mode = Edit> {
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

    pub fn new(
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
        let octave_factor_given = octave_scalar_factor.is_some();
        let note_names_given = note_class_set.is_some();
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
        if let TemperamentType::EqualTemperament(_, o) = n.temperament_type {
            if !octave_factor_given && o.is_finite() && o > 0.0 && o != 1.0 {
                n.octave_scalar_factor = o;
            }
        }
        if !note_names_given && n.note_class_set.len() != n.intervals.len() {
            n.note_class_set = (0..n.intervals.len()).map(|i| i.to_string()).collect();
        }

        n
    }
}

impl<Mode> Temperament<Mode> {
    pub fn temperament_type(&self) -> &TemperamentType {
        &self.temperament_type
    }
    pub fn reference_note(&self) -> usize {
        self.reference_note
    }
    pub fn tuning_hz(&self) -> f64 {
        self.tuning_hz
    }
    pub fn octave_label(&self) -> i64 {
        self.octave_label
    }
    pub fn octave_scalar_factor(&self) -> f64 {
        self.octave_scalar_factor
    }
    pub fn note_class_set(&self) -> &[String] {
        &self.note_class_set
    }
    pub fn intervals(&self) -> &[f64] {
        &self.intervals
    }

    // `intervals[k]` is the position of note class k inside one octave, measured in steps of
    // 1/intervals.len() of that octave, so 12 equal notes are 0.0..=11.0 and `extra` is in the same unit.
    // An index counts note classes upward; `reference_note` sounds at `tuning_hz`.
    pub fn frequency(&self, index: usize, extra: f64) -> Option<f64> {
        let n = self.intervals.len();
        if n == 0 {
            return None;
        }
        let octaves = (index / n) as f64 - (self.reference_note / n) as f64;
        let steps = octaves * n as f64 + self.intervals[index % n]
            - self.intervals[self.reference_note % n]
            + extra;
        Some(self.tuning_hz * self.octave_scalar_factor.powf(steps / n as f64))
    }

    pub fn note_name(&self, index: usize) -> Option<String> {
        let n = self.intervals.len();
        if n == 0 || self.note_class_set.len() != n {
            return None;
        }
        let octave = self.octave_label + (index / n) as i64 - (self.reference_note / n) as i64;
        Some(format!("{}{}", self.note_class_set[index % n], octave))
    }
}

impl Temperament<Edit> {
    pub fn set_tuning_hz(&mut self, tuning_hz: f64) -> bool {
        if tuning_hz.is_finite() && tuning_hz > 0.0 {
            self.tuning_hz = tuning_hz;
            true
        } else {
            false
        }
    }
    pub fn set_reference_note(&mut self, reference_note: usize) -> bool {
        self.reference_note = reference_note;
        true
    }
    pub fn set_octave_label(&mut self, octave_label: i64) -> bool {
        self.octave_label = octave_label;
        true
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
            TemperamentType::EqualTemperament(d, _o) if d.is_finite() && *d >= 1.0 => {
                Some((0..d.round() as usize).map(|i| i as f64).collect())
            }
            // A prime limit does not pick out one scale, so the caller supplies the intervals.
            _ => None,
        }
    }
}
