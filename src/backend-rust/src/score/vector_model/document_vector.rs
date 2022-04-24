use crate::types::*;
use crate::utils;
use std::collections::BTreeMap;
use std::f64;

#[derive(Clone)]
pub struct DocumentVector {
    pub term_weights: TermMap<Weight>,
    pub magnitude: f64
}

pub fn rocchio(v: &DocumentVector, relevant: &[&DocumentVector]) -> DocumentVector {
    let alpha = 0.5;
    let beta = 0.5;

    v.multiply_scalar(alpha).add(&centroid(relevant).multiply_scalar(beta))
}

pub fn centroid(vs: &[&DocumentVector]) -> DocumentVector {
    if vs.len() == 0 { 
        DocumentVector::zero()
    } else {
        let multiplier = 1.0 / vs.len() as f64;

        let v: DocumentVector = 
            vs.iter()
            .fold(DocumentVector::zero(), |v, v_| v.add(*v_));
        
        v.multiply_scalar(multiplier)
    }
}

impl DocumentVector {

    pub fn zero() -> Self {
        Self {
            term_weights: BTreeMap::new(),
            magnitude: 0.0
        }
    }

    pub fn new(term_weights: TermMap<Weight>) -> Self {
        let magnitude = Self::compute_magnitude(&term_weights);
        Self {
            term_weights,
            magnitude
        }
    }

    fn compute_magnitude(term_weights: &TermMap<Weight>) -> f64 {
        f64::sqrt(term_weights.iter().map(|(_, x)| x * x).sum())
    }

    pub fn magnitude(&self) -> f64 { self.magnitude }

    pub fn multiply_scalar(&self, a: f64) -> DocumentVector {
        let term_weights : TermMap<Weight> = self.term_weights
        .iter()
        .map(|(k,v)| (k.clone(), a * v))
        .collect();

        Self::new(term_weights)
    }

    pub fn add(&self, other: &DocumentVector) -> DocumentVector {
        let merged = utils::map::merge_maps(&self.term_weights, &other.term_weights);

        let term_weights: TermMap<Weight> = merged
        .into_iter()
        .map(|res| {
            match res {
                MapMergeResult::Left(k, v) => (k, v),
                MapMergeResult::Right(k, v) => (k, v),
                MapMergeResult::Conflict(k, vl, vr) => (k, vl + vr)
            }
        })
        .collect();

        Self::new(term_weights)
    }

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
