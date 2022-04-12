use crate::types::*;
use crate::utils;
use std::f64;

pub struct DocumentVector {
    pub term_weights: TermMap<Weight>,
    pub magnitude: f64
}

impl DocumentVector {
    pub fn new(term_weights: TermMap<Weight>) -> Self {
        let magnitude = f64::sqrt(term_weights.iter().map(|(_, x)| x * x).sum());
        Self {
            term_weights,
            magnitude
        }
    }

    pub fn magnitude(&self) -> f64 { self.magnitude }

    pub fn dot(&self, other: &DocumentVector) -> f64 {

        let merged = utils::map::merge_maps(&self.term_weights, &other.term_weights);

        merged
        .into_iter()
        .filter_map(|res| {
            if let MapMergeResult::Conflict(_, l, r) = res {
                Some(l * r)
            }
            else {
                None
            }
        })
        .sum::<f64>()
    }

    pub fn cosine_similarity(&self, other: &DocumentVector) -> Score {
        self.dot(other) / (self.magnitude * other.magnitude)
    }
}
