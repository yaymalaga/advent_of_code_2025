use std::{collections::VecDeque, fs::read_to_string};

type Id = u64;

#[derive(Clone)]
struct FreshRange {
    lower_id: Id,
    upper_id: Id,
}

impl FreshRange {
    fn is_id_fresh(&self, id: &Id) -> bool {
        id >= &self.lower_id && id <= &self.upper_id
    }

    fn get_fresh_ids(&self) -> impl Iterator<Item = Id> {
        self.lower_id..=self.upper_id
    }
}

impl From<&str> for FreshRange {
    fn from(value: &str) -> Self {
        let mut ids = value.split('-').take(2);

        let lower_id = ids
            .next()
            .unwrap()
            .parse()
            .expect("Invalid first number in input");

        let upper_id = ids
            .next()
            .unwrap()
            .parse()
            .expect("Invalid second number in input");

        Self { lower_id, upper_id }
    }
}

struct KitchenDB {
    fresh_id_ranges: Vec<FreshRange>,
    ingredients_ids: Vec<Id>,
}

impl KitchenDB {
    fn get_fresh_available_ids(&self) -> impl Iterator<Item = &Id> {
        self.ingredients_ids.iter().filter(|ingredient_id| {
            self.fresh_id_ranges
                .iter()
                .any(|range| range.is_id_fresh(ingredient_id))
        })
    }

    fn get_fresh_ids_count(&self) -> usize {
        KitchenDBUtils::optimize_ranges(&self.fresh_id_ranges)
            .iter()
            .map(|range| range.get_fresh_ids().count())
            .sum()
    }
}

impl From<&str> for KitchenDB {
    fn from(value: &str) -> Self {
        let mut is_ranges_completed = false;
        let mut fresh_id_ranges = Vec::new();
        let mut ingredients_ids = Vec::new();

        for line in value.trim().lines() {
            let line = line.trim();

            if line.is_empty() {
                is_ranges_completed = true;
                continue;
            }

            match is_ranges_completed {
                true => {
                    let id = line.parse().expect("Invalid number");
                    ingredients_ids.push(id);
                }
                false => {
                    let range = FreshRange::from(line);
                    fresh_id_ranges.push(range);
                }
            }
        }

        Self {
            fresh_id_ranges,
            ingredients_ids,
        }
    }
}

struct KitchenDBUtils {}

impl KitchenDBUtils {
    fn optimize_ranges(ranges: &[FreshRange]) -> Vec<FreshRange> {
        let mut ranges = VecDeque::from(ranges.to_owned());

        for _ in 0..ranges.len() {
            let current_range = ranges.pop_front().unwrap();
            let mut is_current_range_merged = false;

            for other_range in ranges.iter_mut() {
                let merged_range = Self::merge_ranges(&current_range, other_range);

                if let Some(new_range) = merged_range {
                    *other_range = new_range;
                    is_current_range_merged = true;
                    break;
                }
            }

            if !is_current_range_merged {
                ranges.push_back(current_range);
            }
        }

        Vec::from(ranges)
    }

    fn merge_ranges(source_range: &FreshRange, target_range: &FreshRange) -> Option<FreshRange> {
        let is_lower_within = (source_range.lower_id >= target_range.lower_id)
            && (source_range.lower_id <= target_range.upper_id);

        let is_upper_within = (source_range.upper_id >= target_range.lower_id)
            && (source_range.upper_id <= target_range.upper_id);

        match (is_lower_within, is_upper_within) {
            (true, true) => Some(target_range.clone()),
            (true, false) => Some(FreshRange {
                lower_id: target_range.lower_id,
                upper_id: source_range.upper_id,
            }),
            (false, true) => Some(FreshRange {
                lower_id: source_range.lower_id,
                upper_id: target_range.upper_id,
            }),
            (false, false) => None,
        }
    }
}

fn main() {
    let data = read_to_string("day_5/data/input.txt").expect("File not found or unabled to read");

    let kitchen_db = KitchenDB::from(data.as_str());

    println!(
        "Day5 -> Part 1: {}",
        kitchen_db.get_fresh_available_ids().count()
    );

    println!("Day5 -> Part 2: {}", kitchen_db.get_fresh_ids_count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let data = r#"
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32"#;

        let kitchen_db = KitchenDB::from(data);

        assert_eq!(kitchen_db.get_fresh_available_ids().count(), 3);

        assert_eq!(kitchen_db.get_fresh_ids_count(), 14);
    }

    #[test]
    fn test_optimize_ranges() {
        let ranges = Vec::from([
            FreshRange {
                lower_id: 12,
                upper_id: 18,
            },
            FreshRange {
                lower_id: 16,
                upper_id: 20,
            },
        ]);

        let ranges = KitchenDBUtils::optimize_ranges(&ranges);

        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].lower_id, 12);
        assert_eq!(ranges[0].upper_id, 20);
    }
}
