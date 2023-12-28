// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}
 
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}
impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut r = &mut list1;
        while list2.is_some() {
            if r.is_none() || list2.as_ref()?.val < r.as_ref()?.val {
                std::mem::swap(r, &mut list2);
            }
            r = &mut r.as_mut()?.next;
        }
        list1
    }
}
