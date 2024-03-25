type ListLink<T> = Option<Box<ListNode<T>>>;

struct ListNode<T> {
    val: T,
    next: ListLink<T>
}

impl<T> ListNode<T> {
    fn new(val: T) -> Self {
        ListNode {
            val,
            next: None
        }
    }
}

impl<T> Into<ListLink<T>> for ListNode<T> {
    fn into(self) -> ListLink<T> {
        Some(Box::new(self))
    }
}

pub struct LinkedList<T> {
    head: ListLink<T>
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None
        }
    }

    pub fn insert(&mut self, val: T) {
        let mut curr_node = self.head.as_deref_mut();

        while let Some(next) = curr_node.map(|n| n.next.as_deref_mut()) {
            curr_node = next;
        }

        curr_node.map(|n| {
            n.next.insert(ListNode::new(val).into());
            n
        });

    }
}

fn main() {}