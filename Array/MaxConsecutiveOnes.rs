//! 485. 最大连续1的个数
//! 给定一个二进制数组， 计算其中最大连续1的个数。
//! 
//! 示例 1:
//! 
//! 输入: [1,1,0,1,1,1]
//! 输出: 3
//! 解释: 开头的两位和最后的三位都是连续1，所以最大连续1的个数是 3.
//! 注意：
//! 
//! 输入的数组只包含 0 和1。
//! 输入数组的长度是正整数，且不超过 10,000。

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut ret: i32 = 0;
        let mut max: i32 = 0;

        for num in nums {
            if num == 1 {
                max = max + 1;
                if max > ret {
                    ret = max;
                }
            } else {
                max = 0;
            }
        }

        return ret;
    }
}