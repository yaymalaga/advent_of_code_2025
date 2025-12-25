use std::fs::read_to_string;

type Id = u64;

struct ProductRangesParser {}

impl ProductRangesParser {
    pub fn parse(data: &str) -> impl Iterator<Item = Id> {
        data.split(',')
            .flat_map(|range| ProductRange::from(range).generate_ids())
    }
}

struct ProductRange {
    first_id: Id,
    last_id: Id,
}

impl ProductRange {
    fn generate_ids(self) -> impl Iterator<Item = Id> {
        self.first_id..=self.last_id
    }
}

impl From<&str> for ProductRange {
    fn from(value: &str) -> Self {
        let mut ids = value.split('-').take(2);

        let first_id = ids
            .next()
            .unwrap()
            .parse()
            .expect("Invalid first number in input");

        let last_id = ids
            .next()
            .unwrap()
            .parse()
            .expect("Invalid second number in input");

        Self { first_id, last_id }
    }
}

trait IdValidation {
    fn get_groups_sizes(id: &str) -> impl Iterator<Item = usize>;

    fn is_valid(id: &Id) -> bool {
        let id = id.to_string();

        // Pattern must be repeated at least twice
        if id.len() == 1 {
            return true;
        }

        for group_size in Self::get_groups_sizes(&id) {
            let mut grouped_data = id.as_bytes().chunks(group_size);

            let first_item = grouped_data.next().unwrap();
            let is_all_matching = grouped_data.all(|group| group == first_item);

            if is_all_matching {
                return false;
            }
        }

        true
    }

    fn find_invalid_ids(ids: &[Id]) -> impl Iterator<Item = &Id> {
        ids.iter().filter(|&id| !Self::is_valid(id))
    }
}

struct BasicIdValidator {}

impl IdValidation for BasicIdValidator {
    fn get_groups_sizes(id: &str) -> impl Iterator<Item = usize> {
        [(id.len() as f64 / 2.)]
            .into_iter()
            .filter(|size| size.fract() == 0.0)
            .map(|size| size as usize)
    }
}

struct StrictIdValidator {}

impl IdValidation for StrictIdValidator {
    fn get_groups_sizes(id: &str) -> impl Iterator<Item = usize> {
        let max_size = id.len() / 2;

        (1..=max_size).filter(|&size| id.len().is_multiple_of(size)) // TODO: how? move isn't taking ownership of id?
    }
}

fn main() {
    let data = read_to_string("day_2/data/input.txt").expect("File not found or unabled to read");

    let products_ids: Vec<Id> = ProductRangesParser::parse(&data).collect();

    let invalid_ids_sum: u64 = BasicIdValidator::find_invalid_ids(&products_ids).sum();
    println!("Day2 -> Part1: {}", invalid_ids_sum);

    let invalid_ids_strict_sum: u64 = StrictIdValidator::find_invalid_ids(&products_ids).sum();
    println!("Day2 -> Part2: {}", invalid_ids_strict_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRODUCT_RANGE: ProductRange = ProductRange {
        first_id: 95,
        last_id: 115,
    };

    #[test]
    fn parse_product_range() {
        let product_range_raw = "95-115";
        let product_range = ProductRange::from(product_range_raw);

        assert_eq!(product_range.first_id, 95);
        assert_eq!(product_range.last_id, 115);
    }

    #[test]
    fn generate_ids() {
        let ids: Vec<Id> = PRODUCT_RANGE.generate_ids().collect();

        assert_eq!(ids.len(), 21);
        assert_eq!(*ids.first().unwrap(), 95);
        assert_eq!(*ids.get(1).unwrap(), 96);
        assert_eq!(*ids.iter().nth_back(1).unwrap(), 114);
        assert_eq!(*ids.last().unwrap(), 115);
    }

    #[test]
    fn get_groups_sizes() {
        let id = "12341234";

        let groups_sizes_basic: Vec<usize> = BasicIdValidator::get_groups_sizes(id).collect();

        assert_eq!(groups_sizes_basic, vec![4]);

        let groups_sizes_strict: Vec<usize> = StrictIdValidator::get_groups_sizes(id).collect();

        assert_eq!(groups_sizes_strict.len(), 3);
        assert!(groups_sizes_strict.contains(&1));
        assert!(groups_sizes_strict.contains(&2));
        assert!(groups_sizes_strict.contains(&4));

        let id = "222222222";

        let groups_sizes_basic: Vec<usize> = BasicIdValidator::get_groups_sizes(id).collect();

        assert!(groups_sizes_basic.is_empty());

        let groups_sizes_strict: Vec<usize> = StrictIdValidator::get_groups_sizes(id).collect();

        assert_eq!(groups_sizes_strict.len(), 2);
        assert!(groups_sizes_strict.contains(&1));
        assert!(groups_sizes_strict.contains(&3));
    }

    #[test]
    fn check_id_simple() {
        assert!(BasicIdValidator::is_valid(&PRODUCT_RANGE.first_id));
        assert!(BasicIdValidator::is_valid(&PRODUCT_RANGE.last_id));

        assert!(!BasicIdValidator::is_valid(&99));
        assert!(BasicIdValidator::is_valid(&111));
    }

    #[test]
    fn find_invalid_ids_simple() {
        let product_range = ProductRange {
            first_id: 11,
            last_id: 22,
        };

        let ids: Vec<Id> = product_range.generate_ids().collect();

        let invalid_ids: Vec<&Id> = BasicIdValidator::find_invalid_ids(&ids).collect();

        assert_eq!(invalid_ids, Vec::from([&11, &22]));
    }

    #[test]
    fn check_id_strict() {
        assert!(StrictIdValidator::is_valid(&PRODUCT_RANGE.first_id));
        assert!(StrictIdValidator::is_valid(&PRODUCT_RANGE.last_id));

        assert!(!StrictIdValidator::is_valid(&99));
        assert!(!StrictIdValidator::is_valid(&111));

        assert!(!StrictIdValidator::is_valid(&565656));
        assert!(!StrictIdValidator::is_valid(&446446));
    }

    #[test]
    fn find_invalid_ids_strict() {
        let ids: Vec<Id> = PRODUCT_RANGE.generate_ids().collect();

        let invalid_ids: Vec<&Id> = StrictIdValidator::find_invalid_ids(&ids).collect();

        assert_eq!(invalid_ids, Vec::from([&99, &111]));
    }

    #[test]
    fn check_example_input() {
        let example_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let products_ids: Vec<Id> = ProductRangesParser::parse(example_input).collect();

        let invalid_ids_basic = BasicIdValidator::find_invalid_ids(&products_ids);
        assert_eq!(invalid_ids_basic.sum::<u64>(), 1227775554);

        let invalid_ids_strict = StrictIdValidator::find_invalid_ids(&products_ids);
        assert_eq!(invalid_ids_strict.sum::<u64>(), 4174379265);
    }
}
