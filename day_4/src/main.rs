use std::fs::read_to_string;

#[derive(PartialEq, Debug)]
enum GridType {
    Empty,
    PaperRoll,
}

impl From<&char> for GridType {
    fn from(value: &char) -> Self {
        match value {
            '.' => Self::Empty,
            '@' => Self::PaperRoll,
            _ => panic!("Invalid char received"),
        }
    }
}

struct Grid {
    n_rows: usize,
    n_columns: usize,
    data: Vec<Vec<GridType>>,
}

impl Grid {
    fn is_within_bounds(&self, row: isize, col: isize) -> bool {
        row >= 0 && row < self.n_rows as isize && col >= 0 && col < self.n_columns as isize
    }

    fn is_paper_roll(&self, row: usize, col: usize) -> bool {
        self.data[row][col] == GridType::PaperRoll
    }

    fn remove_item(&mut self, row: usize, col: usize) {
        self.data[row][col] = GridType::Empty;
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let grid_data: Vec<Vec<GridType>> = value
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|item| GridType::from(&item))
                    .collect()
            })
            .collect();

        Self {
            n_rows: grid_data.len(),
            n_columns: grid_data.first().unwrap_or(&Vec::new()).len(),
            data: grid_data,
        }
    }
}

struct ForkLiftsHelper<'a> {
    grid: &'a mut Grid,
}

impl<'a> ForkLiftsHelper<'a> {
    const ADJACENT_OFFSETS: [[isize; 2]; 8] = [
        [1, 0],
        [0, 1],
        [1, 1],
        [-1, 0],
        [0, -1],
        [-1, -1],
        [-1, 1],
        [1, -1],
    ];

    fn new(grid: &'a mut Grid) -> Self {
        Self { grid }
    }
    fn is_item_accessible(&self, row: usize, col: usize) -> bool {
        if !self.grid.is_paper_roll(row, col) {
            return false;
        }

        Self::ADJACENT_OFFSETS
            .iter()
            .map(|[row_offset, col_offset]| [row as isize + row_offset, col as isize + col_offset])
            .filter(|&[row, col]| {
                self.grid.is_within_bounds(row, col)
                    && self.grid.is_paper_roll(row as usize, col as usize)
            })
            .count()
            < 4
    }

    fn find_accessible_paper_rolls(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.grid.n_rows)
            .flat_map(|row_index| {
                (0..self.grid.n_columns)
                    .map(|col_index| (row_index, col_index))
                    .collect::<Vec<(usize, usize)>>()
            })
            .filter(|&(row, col)| self.is_item_accessible(row, col))
    }

    fn iterative_remove_accessible_paper_rolls(&mut self) -> usize {
        let mut removed_paper_rolls = 0;

        loop {
            let valid_paper_rolls: Vec<(usize, usize)> =
                self.find_accessible_paper_rolls().collect();

            if valid_paper_rolls.is_empty() {
                return removed_paper_rolls;
            }

            for &(row, col) in valid_paper_rolls.iter() {
                self.grid.remove_item(row, col);
            }

            removed_paper_rolls += valid_paper_rolls.len();
        }
    }
}

fn main() {
    let data = read_to_string("day_4/data/input.txt").expect("File not found or unabled to read");

    let mut grid = Grid::from(data.as_str());
    let mut forklifts_helper = ForkLiftsHelper::new(&mut grid);

    println!(
        "Day4 -> Part 1: {}",
        forklifts_helper.find_accessible_paper_rolls().count()
    );

    println!(
        "Day4 -> Part 2: {}",
        forklifts_helper.iterative_remove_accessible_paper_rolls()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let data = r#"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@."#;

        let mut grid = Grid::from(data);
        let mut forklifts_helper = ForkLiftsHelper::new(&mut grid);

        let valid_paper_rolls = forklifts_helper.find_accessible_paper_rolls();

        assert_eq!(valid_paper_rolls.count(), 13);

        let removed_paper_rolls = forklifts_helper.iterative_remove_accessible_paper_rolls();

        assert_eq!(removed_paper_rolls, 43);
    }
}
