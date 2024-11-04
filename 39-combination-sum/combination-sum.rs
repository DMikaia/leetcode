impl Solution {
    fn find_combinations(index: usize, target: i32, candidates: &Vec<i32>, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(current.clone());
            return;
        }

        for i in index..candidates.len() {
            if candidates[i] <= target {
                current.push(candidates[i]);

                Self::find_combinations(
                    i,
                    target - candidates[i],
                    candidates,
                    current,
                    result,
                );

                current.pop();
            }
        }
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::new();       

        Self::find_combinations(0, target, &candidates, &mut current, &mut result);

        result
    }
}