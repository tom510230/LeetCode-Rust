//! 151. 翻转字符串里的单词
//! 给定一个字符串，逐个翻转字符串中的每个单词。
//! 
//! 示例 1：
//! 
//! 输入: "the sky is blue"
//! 输出: "blue is sky the"
//! 示例 2：
//! 
//! 输入: "  hello world!  "
//! 输出: "world! hello"
//! 解释: 输入字符串可以在前面或者后面包含多余的空格，但是反转后的字符不能包括。
//! 示例 3：
//! 
//! 输入: "a good   example"
//! 输出: "example good a"
//! 解释: 如果两个单词间有多余的空格，将反转后单词间的空格减少到只含一个。
//! 
//! 说明：
//! 
//! 无空格字符构成一个单词。
//! 输入字符串可以在前面或者后面包含多余的空格，但是反转后的字符不能包括。
//! 如果两个单词间有多余的空格，将反转后单词间的空格减少到只含一个。

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut res = "".to_string();
        let words:Vec<&str> = s.split(' ').collect();
        let mut flag = 0;
        for i in (0..words.len()).rev(){
            if words[i] == ""{ continue;}
            if flag == 1{
                res.push(' ');
            }
            flag = 1;
            res.push_str(words[i]);
            
        }

        return res;   
    }
}