use std::collections::{HashMap, HashSet};

impl Solution {
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: HashMap<i32, HashSet<char>> = HashMap::new();
        let mut cols: HashMap<i32, HashSet<char>> = HashMap::new();
        let mut sub_boxes: HashMap<i32, HashSet<char>> = HashMap::new();

        for r in 0..9 {
            for c in 0..9 {
                let cell = board[r][c];
                if cell == '.' {
                    continue;
                }

                let sub_box_index: i32 = (r as i32 / 3) * 3 + (c as i32 / 3);

                if rows.get(&(r as i32)).map_or(false, |set| set.contains(&cell))
                    || cols.get(&(c as i32)).map_or(false, |set| set.contains(&cell))
                    || sub_boxes.get(&sub_box_index).map_or(false, |set| set.contains(&cell)) {
                    return false;
                }

                rows.entry(r as i32).or_insert(HashSet::new()).insert(cell);
                cols.entry(c as i32).or_insert(HashSet::new()).insert(cell);
                sub_boxes.entry(sub_box_index).or_insert(HashSet::new()).insert(cell);
            }
        }

        true
    }
}