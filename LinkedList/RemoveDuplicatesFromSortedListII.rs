//! 82. 删除排序链表中的重复元素 II
//! 给定一个排序链表，删除所有含有重复数字的节点，只保留原始链表中 没有重复出现 的数字。
//! 
//! 示例 1:
//! 
//! 输入: 1->2->3->3->4->4->5
//! 输出: 1->2->5
//! 示例 2:
//! 
//! 输入: 1->1->1->2->3
//! 输出: 2->3

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
use std::collections::HashMap;
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.clone().unwrap().next.is_none() {
            return head;
        }

        let mut node = head.clone();
        let mut map: [i32; 2147483647] = [0; 2147483647];
        let mut counter = HashMap::new();

        // 记录重复的val值
        while node.is_some() {
            let node_unwrap = node.unwrap();
            counter.entry(node_unwrap.val).and_modify(|e| *e += 1).or_insert(1);
            node = node_unwrap.next;
        }
        
        // 定下head的起始点
        let mut check = head.clone();
        while check.is_some() {
            let check_unwrap = check.unwrap();
            if counter[&check_unwrap.val] > 1 {
                head = head.unwrap().next;
            } else {
                break;
            }
            check = check_unwrap.next;
        }

        // 跳过要删除的结点，处理next的指向
        let mut p = &mut head;
        while let Some(now) = p {
            while let Some(next) = now.next.as_mut() {
                if counter[&next.val] == 1 {
                    break;
                }
                let next_next = std::mem::replace(&mut next.next, None);
                std::mem::replace(&mut now.next, next_next);
            }
            p = &mut now.next;
        }
        return head;
    }

    // 递归的例子如下
    // pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     Self::delete_duplicates_node(head, false)
    // }
    // pub fn delete_duplicates_node(head: Option<Box<ListNode>>, delete_current: bool) -> Option<Box<ListNode>> {
    //     match head {
    //         Some(mut node) => {
    //             let n = node.next;
    //             if n.is_none() || n.as_ref().unwrap().val != node.val {
    //                 let new_next =
    //                     Self::delete_duplicates_node(n, false);
    //                 if delete_current {
    //                     new_next
    //                 } else {
    //                     node.next = new_next;
    //                     Some(node)
    //                 }
    //             } else {
    //                 Self::delete_duplicates_node(n, true)
    //             }
    //         }
    //         _ => None
    //     }
    // }
}