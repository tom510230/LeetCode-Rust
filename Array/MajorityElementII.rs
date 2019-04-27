//! 229. 求众数 II
//! 给定一个大小为 n 的数组，找出其中所有出现超过 ⌊ n/3 ⌋ 次的元素。
//! 
//! 说明: 要求算法的时间复杂度为 O(n)，空间复杂度为 O(1)。
//! 
//! 示例 1:
//! 
//! 输入: [3,2,3]
//! 输出: [3]
//! 示例 2:
//! 
//! 输入: [1,1,1,3,3,2,2,2]
//! 输出: [1,2]

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        // 在任何数组中，出现次数大于该数组长度1/3的值最多只有两个。因此只需要定义两个临时变量记录即可

        let mut count1: i32 = 0;
        let mut result1: i32 = 0;
        let mut count2: i32 = 0;
        let mut result2: i32 = 0;
        let mut size: i32 = (nums.len() / 3) as i32;
        let mut ret: Vec<i32> = vec![];

        if nums.len() == 0 {
            return ret;
        }

        for num in nums.iter() {
            if ((count1 == 0 || result1 == *num) && *num != result2) {
                count1 = count1 + 1;
                result1 = *num;
            } else if count2 == 0 || result2 == *num {
                count2 = count2 + 1;
                result2 = *num;
            } else {
                count1 = count1 - 1;
                count2 = count2 - 1;
            }
        }

        count1 = 0;
        count2 = 0;

        for num in nums.iter() {
            if *num == result1 {
                count1 = count1 + 1;
            } else if *num == result2 {
                count2 = count2 + 1;
            }
        }

        if count1 > size { ret.push(result1); }
        if count2 > size { ret.push(result2); }

        return ret;

        // 本题还有HashMap的解题方法，代码比较短，但是需要仔细了解map的每个api
        // let required_times = nums.len() / 3 + 1;
        // let mut counter = HashMap::new();
        
        // for num in nums {
        //     counter.entry(num).and_modify(|e| *e += 1).or_insert(1);
        // }
        
        // counter.iter().filter(|&(_, v)| *v >= required_times).map(|(&k, _)| k).collect()
    }
}