impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; nums.len()];;

        let mut prefix = 1;
        for i in 0..nums.len() {
            res[i] = prefix;
            prefix *= nums[i];
        }

        let mut postfix = 1;
        for i in (0..res.len()).rev() {
            res[i] *= postfix;
            postfix *= nums[i];
        }

        res
    }
}