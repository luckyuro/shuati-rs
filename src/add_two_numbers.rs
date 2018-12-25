#[derive(PartialEq, Eq, Debug)]
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

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut l1, mut l2) = (l1, l2);
    let mut ret: Option<Box<ListNode>> = None;
    let mut tail =  &mut ret;
    let mut carry: i32 = 0;

    loop {

        let (v1, l1_c): (i32, Option<Box<ListNode>>) = match l1 {
            Some(x) => (x.val, x.next),
            None => (0, None)
        };
        let (v2, l2_c): (i32, Option<Box<ListNode>>) = match l2 {
            Some(x) => (x.val, x.next),
            None => (0, None)
        };

        let sum = v1 + v2 + carry;
        carry = sum / 10;

        if tail == &mut None {
            ret = Some(Box::new(ListNode::new(sum % 10)));
            tail = &mut ret;
        } else {
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
            tail = &mut tail.as_mut().unwrap().next;
        }

        if l1_c == None && l2_c == None && carry == 0{
            break;
        }

        l1 = l1_c;
        l2 = l2_c;
    }

    ret
}


#[cfg(test)]
mod test {
    use super::add_two_numbers;

    #[test]
    fn test_add_two_numbers() {
    }
}