/*
 * @lc app=leetcode.cn id=148 lang=rust
 *
 * [148] 排序链表
 */
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;
// @lc code=start
impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut length = 0;
        let mut p = head.as_ref();

        while let Some(n) = p {
            p = n.next.as_ref();
            length += 1;
        }

        let mut sub_length = 1;
        let mut res = None;

        while sub_length < length {
            let mut cur = &mut head;
            let mut p = &mut res;
            while cur.is_some() {
                let left = Self::cut(&mut cur, sub_length);
                let right = Self::cut(&mut cur, sub_length);
                *p = Self::merge(left, right);
                while let Some(ln) = p {
                    p = &mut ln.next;
                }
            }
            sub_length *= 2;
            head = res.take();
        }
        head
    }
    fn merge(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut res = None;
        let mut p = &mut res;
        while l1.is_some() && l2.is_some() {
            let v1 = l1.as_ref().unwrap();
            let v2 = l2.as_ref().unwrap();
            let val = if v1.val < v2.val {
                let mut val = l1.unwrap();
                l1 = val.next.take();
                val
            } else {
                let mut val = l2.unwrap();
                l2 = val.next.take();
                val
            };
            *p = Some(val);
            while let Some(ln) = p {
                p = &mut ln.next;
            }
        }
        *p = if l1.is_some() { l1 } else { l2 };
        res
    }
    fn cut(head: &mut Option<Box<ListNode>>, mut n: i32) -> Option<Box<ListNode>> {
        let mut res = None;
        let mut p = &mut res;
        while n > 0 && head.is_some() {
            if let Some(mut ln) = head.take() {
                *head = ln.next.take();
                *p = Some(ln);
                while let Some(ln2) = p {
                    p = &mut ln2.next;
                }
            }
            n -= 1;
        }
        res
    }
}
// @lc code=end
