#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn _set_next(&mut self, node: Self) {
        self.next = Some(Box::new(node));
    }
    fn _get_last(&mut self) -> &mut Self {
        if let Some(ref mut node) = self.next {
            return node._get_last();
        }
        self
    }
    fn add_tail(&mut self, node: Self) {
        self._get_last()._set_next(node);
    }
}

impl IntoIterator for ListNode {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        let mut _vec: Vec<i32> = vec![];
        let mut _next = self.next;
        _vec.push(self.val);
        loop {
            match _next {
                Some(node) => {
                    _vec.push(node.val);
                    _next = node.next;
                }
                None => break,
            }
        }
        _vec.into_iter()
    }
}

impl std::fmt::Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut list_str = String::new();
        let mut _next = &self.next;

        list_str += &format!("{} -> ", self.val);
        loop {
            match _next {
                Some(node) => {
                    list_str += &format!("{} -> ", node.val);
                    _next = &node.next;
                }
                None => break,
            }
        }
        write!(f, "{}", list_str)
    }
}

/// 输入：`l1 = [2,4,3], l2 = [5,6,4]`  
/// 输出：`[7,0,8]`  
/// 解释：`342 + 465 = 807.`
#[allow(dead_code)]
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (None, Some(node)) | (Some(node), None) => Some(node),
        (Some(node1), Some(node2)) => {
            let mut node1_iter = node1.into_iter();
            let mut node2_iter = node2.into_iter();
            let mut add_flag = 0;
            let mut result_list: Option<Box<ListNode>> = None;
            loop {
                match (node1_iter.next(), node2_iter.next()) {
                    (None, Some(v)) | (Some(v), None) => {
                        let sum = v + add_flag;
                        if sum == 10 {
                            add_flag = 1
                        } else {
                            add_flag = 0
                        }
                        let sum_node = ListNode::new(sum % 10);
                        match result_list {
                            Some(ref mut node) => node.add_tail(sum_node),
                            None => result_list = Some(Box::new(sum_node)),
                        }
                    }
                    (Some(v1), Some(v2)) => {
                        let sum = v1 + v2 + add_flag;
                        let sum_node = ListNode::new(sum % 10);
                        if sum > 9 {
                            add_flag = 1
                        } else {
                            add_flag = 0
                        };
                        match result_list {
                            Some(ref mut node) => node.add_tail(sum_node),
                            None => result_list = Some(Box::new(sum_node)),
                        }
                    }
                    (None, None) => {
                        if add_flag == 1 {
                            if let Some(ref mut node) = result_list {
                                node.add_tail(ListNode::new(1))
                            }
                        }
                        break;
                    }
                }
            }
            result_list
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{add_two_numbers, ListNode};
    #[test]
    fn it_works() {
        let mut l1 = ListNode::new(3);
        l1.add_tail(ListNode::new(7));

        let mut l2 = ListNode::new(9);
        l2.add_tail(ListNode::new(2));

        println!("{}\n{}", &l1, &l2);
        match add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2))) {
            Some(sum_node) => {
                println!("{}", sum_node);
            }
            None => panic!("error"),
        };
    }
}
