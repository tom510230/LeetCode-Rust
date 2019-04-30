//! 206. 反转链表
//! 反转一个单链表。
//! 
//! 示例:
//! 
//! 输入: 1->2->3->4->5->NULL
//! 输出: 5->4->3->2->1->NULL
//! 进阶:
//! 你可以迭代或递归地反转链表。你能否用两种方法解决这道题？


// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut temp: Option<Box<ListNode>> = None;
        let mut next;
        let mut first = head;
        while first.is_some() {
            let mut head_unwraped = first.unwrap();
            next = head_unwraped.next;
            head_unwraped.next = temp;
            temp = Some(head_unwraped);
            first = next;
        }
        return temp;

        // 附上递归的写法，利用了函数式编程的特点
        // fn reverse(
        //     node: Option<Box<ListNode>>,
        //     tail: Option<Box<ListNode>>,
        // ) -> Option<Box<ListNode>> {
        //     match node {
        //         Some(mut node) => {
        //             let next = std::mem::replace(&mut node.next, tail);
        //             reverse(next, Some(node))
        //         }
        //         None => tail,
        //     }
        // }
        // reverse(head, None)
    }
}