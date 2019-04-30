//! 234. 回文链表
//! 请判断一个链表是否为回文链表。
//! 
//! 示例 1:
//! 
//! 输入: 1->2
//! 输出: false
//! 示例 2:
//! 
//! 输入: 1->2->2->1
//! 输出: true
//! 进阶：
//! 你能否用 O(n) 时间复杂度和 O(1) 空间复杂度解决此题？

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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() || head.clone().unwrap().next.is_none() {
            return head;
        }

        let mut node = head.clone();
        let mut map: [i32; 10000] = [0; 10000];

        while node.is_some() {
            let node_unwrap = node.unwrap();
            map[node_unwrap.val as usize] += 1;
            node = node_unwrap.next;
        }
        
        let mut h = head.clone();
        let mut ret: Option<Box<ListNode>> = None;
        let mut temp: Option<Box<ListNode>> = None;
        while h.is_some() {
            let node_unwrap = h.clone().unwrap();
            if map[node_unwrap.val as usize] == 1 {
                if ret.is_none() {
                    temp = h;
                    ret = temp;
                } else {
                    let mut ret_unwrap = temp.unwrap();
                    ret_unwrap.next = h;
                    temp = ret_unwrap.next;
                }
            }
            h = node_unwrap.next;
        }

        return ret;
    }

    // 以下是不占用额外存储空间的解法
    // pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
    //     if head.is_none() {
    //         return true;
    //     }
    //     let mut count = 0;
    //     {
    //         let mut cur = head.as_ref();
    //         while let Some(node) = cur.take() {
    //             count += 1;
    //             cur = node.next.as_ref();
    //         }
    //     }
    //     if count < 2 { return true; }
    //     count = if count % 2 == 0 {
    //         count/2
    //     } else {
    //         count/2+1
    //     };
    //     let mut mid = &mut head;
    //     for _ in 0..count {
    //         mid = &mut mid.as_mut().unwrap().next;
    //     }
    //     let mut cur = mid.take();
    //     let mut prev = None;
    //     while let Some(mut cur_inner) = cur.take() {
    //         let next = cur_inner.next.take();
    //         cur_inner.next = prev.take();
    //         prev = Some(cur_inner);
    //         cur = next;
    //     }
    //     let mut t_head = prev.as_ref();
    //     let mut p_head = head.as_ref();
    //     while let Some(t) = t_head.take()  {
    //         if let Some(p) = p_head.take() {
    //             if p.val != t.val {
    //                 return false;
    //             } 
    //             p_head = p.next.as_ref(); 
    //         } else {
    //             return false;
    //         }
    //         t_head = t.next.as_ref();
    //     }
    //     true

    // }

}