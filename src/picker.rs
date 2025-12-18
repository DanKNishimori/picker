use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;

pub struct Picker<'a> {
    values: Vec<&'a str>,
    dist: WeightedIndex<f32>,
}

impl<'a> Picker<'a> {
    pub fn new(values: Vec<&'a str>, weights: Vec<f32>) -> Self {
        let dist = WeightedIndex::new(weights).unwrap();
        Self { values, dist }
    }

    pub fn draw(&self) -> &'a str {
        let index = self.dist.sample(&mut rand::rng());
        self.values
            .get(index)
            .expect("unexpected error of bad indexing")
    }
}
