use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;

use std::num::ParseFloatError;
use std::str::FromStr;

const NULL_ENTRY: (String, f32) = (String::new(), 0.0);
const IGNORE_TAGS: [&str; 5] = ["#ignore", "#...", "#title", "#comment", "#label"];

pub struct Picker {
    values: Vec<String>,
    dist: WeightedIndex<f32>,
}

impl Picker {
    pub fn new(values: Vec<String>, weights: Vec<f32>) -> Self {
        let dist = WeightedIndex::new(weights).unwrap();
        Self { values, dist }
    }

    pub fn draw(&self) -> &str {
        let index = self.dist.sample(&mut rand::rng());
        self.values
            .get(index)
            .expect("unexpected error of bad indexing")
    }
}

impl FromStr for Picker {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut current_weight: Option<f32> = None;

        let (values, weights): (Vec<String>, Vec<f32>) = s
            .lines()
            .map(|line| {
                let (entry, weight_str) = get_entry(line);
                let weight = parse_weight_value(weight_str);

                if entry.trim().is_empty() && !weight_str.is_empty() {
                    current_weight = Some(weight);
                    return NULL_ENTRY;
                }

                let block_marker = get_indentation(entry);

                if have_ignore_tags(line) {
                    return NULL_ENTRY;
                }

                if let Some(dash) = block_marker {
                    let clean_entry = entry.strip_prefix(dash).unwrap().trim().to_string();
                    (clean_entry, current_weight.unwrap_or(weight))
                } else {
                    current_weight = None;
                    (entry.trim().to_string(), weight)
                }
            })
            .collect();

        Ok(Picker::new(values, weights))
    }
}

fn get_indentation(entry: &str) -> Option<&str> {
    match &entry[0..3] {
        c if c.starts_with(" - ") || c.starts_with(" — ") => Some(c),
        _ => None,
    }
}

fn parse_weight_value(weight_str: &str) -> f32 {
    weight_str
        .strip_prefix("#weight ")
        .expect("#weight need to be followed by a float number.")
        .parse::<f32>()
        .unwrap_or(1.0)
}

fn get_entry(line: &str) -> (&str, &str) {
    if let Some(idx) = line.find("#weight") {
        return line.split_at(idx);
    }
    (line, "#weight 1.0")
}
fn have_ignore_tags(line: &str) -> bool {
    IGNORE_TAGS
        .iter()
        .map(|tag| line.ends_with(*tag))
        .collect::<Vec<bool>>()
        .contains(&true)
}
