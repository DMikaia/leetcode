class Solution {
    public boolean containsNearbyDuplicate(int[] nums, int k) {
        Set<Integer> window = new HashSet<Integer>();
        int left = 0;

        for (int right = 0; right < nums.length; right++) {
            if (right - left > k) {
                window.remove(nums[left]);
                left++;
            }
            if (window.contains(nums[right])) {
                return true;
            }
            window.add(nums[right]);
        }
        
        return false;
    }
}