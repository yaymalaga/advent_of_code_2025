use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Joltage = u64;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Battery {
    joltage: Joltage,
}

impl From<&char> for Battery {
    fn from(value: &char) -> Self {
        match *value >= '1' && *value <= '9' {
            true => {
                let joltage = value.to_digit(10).unwrap() as u64;

                Self { joltage }
            }
            false => panic!("Invalid digit between 1-9"),
        }
    }
}

struct BatteryBank {
    batteries: Vec<Battery>,
}

impl From<&str> for BatteryBank {
    fn from(value: &str) -> Self {
        let batteries = value.chars().map(|char| Battery::from(&char)).collect();

        Self { batteries }
    }
}

impl BatteryBank {
    fn get_max_joltage_battery(batteries: &[Battery]) -> &Battery {
        batteries.iter().max().unwrap()
    }

    fn get_max_joltage_combination(&self, combination_size: usize) -> Joltage {
        let mut batteries_joltage_taken = Vec::with_capacity(combination_size);
        let mut last_taken_battery_index = None;

        while batteries_joltage_taken.len() < combination_size {
            // Start after the previous match
            let initial_index = match last_taken_battery_index {
                Some(index) => index + 1,
                None => 0,
            };

            // Ensure enough space to pick the total combination size
            let final_index =
                self.batteries.len() - (combination_size - batteries_joltage_taken.len());

            let max_joltage_battery =
                Self::get_max_joltage_battery(&self.batteries[initial_index..=final_index]);

            batteries_joltage_taken.push(max_joltage_battery.joltage);

            last_taken_battery_index = Some(
                self.batteries
                    .iter()
                    .enumerate()
                    .skip(initial_index)
                    .find(|(_, battery)| battery.joltage == max_joltage_battery.joltage)
                    .unwrap()
                    .0,
            );
        }

        // Create the number from digits concatenation
        batteries_joltage_taken
            .iter()
            .fold(0, |acc, joltage| acc * 10 + joltage)
    }
}

fn main() {
    let file = File::open("day_3/data/input.txt").unwrap();

    let batteries_banks: Vec<BatteryBank> = BufReader::new(file)
        .lines()
        .map(|data| BatteryBank::from(data.unwrap().as_str()))
        .collect();

    println!(
        "Day3 -> Part 1: {}",
        batteries_banks
            .iter()
            .map(|battery_bank| battery_bank.get_max_joltage_combination(2))
            .sum::<u64>()
    );

    println!(
        "Day3 -> Part 2: {}",
        batteries_banks
            .iter()
            .map(|battery_bank| battery_bank.get_max_joltage_combination(12))
            .sum::<u64>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_battery_bank() {
        let data = [9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];

        let batteries: Vec<Battery> = data.iter().map(|item| Battery { joltage: *item }).collect();
        let raw_data: String = data.iter().map(|item| item.to_string()).collect();

        assert_eq!(BatteryBank::from(raw_data.as_str()).batteries, batteries);
    }

    #[test]
    fn get_max_joltage_combination() {
        let battery_bank_raw = "987654321111111";
        assert_eq!(
            BatteryBank::from(battery_bank_raw).get_max_joltage_combination(2),
            98
        );
        assert_eq!(
            BatteryBank::from(battery_bank_raw).get_max_joltage_combination(12),
            987654321111
        );

        let battery_bank_raw = "811111111111119";
        assert_eq!(
            BatteryBank::from(battery_bank_raw).get_max_joltage_combination(2),
            89
        );
        assert_eq!(
            BatteryBank::from(battery_bank_raw).get_max_joltage_combination(12),
            811111111119
        );

        let battery_bank_raw = "234234234234278";
        assert_eq!(
            BatteryBank::from(battery_bank_raw).get_max_joltage_combination(2),
            78
        );
        assert_eq!(
            BatteryBank::from(battery_bank_raw).get_max_joltage_combination(12),
            434234234278
        );

        let battery_bank_raw = "818181911112111";
        assert_eq!(
            BatteryBank::from(battery_bank_raw).get_max_joltage_combination(2),
            92
        );
        assert_eq!(
            BatteryBank::from(battery_bank_raw).get_max_joltage_combination(12),
            888911112111
        );
    }
}
