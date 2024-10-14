impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![1; nums.len()];;

        for i in 1..nums.len() {
            res[i] = res[i - 1] * nums[i - 1];
        }

        let mut postfix = 1;
        for i in (0..res.len()).rev() {
            res[i] *= postfix;
            postfix *= nums[i];
        }

        res
    }
}