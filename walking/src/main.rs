use std::{marker::PhantomData, ptr::{self, NonNull}, cell::Cell};
use ::polonius_the_crab::prelude::*;
use recursive_reference::*;

type ListLink<T> = Option<Box<ListNode<T>>>;

struct ListNode<T> {
    val: T,
    next: ListLink<T>,
}

impl<T> ListNode<T> {
    fn new(val: T, next: ListLink<T>) -> Self {
        ListNode { val, next }
    }
}

impl<T> Into<ListLink<T>> for ListNode<T> {
    fn into(self) -> ListLink<T> {
        Some(Box::new(self))
    }
}

pub struct LinkedList<T> {
    head: ListLink<T>,
}

struct ListWalker<'list, T> {
    curr_ptr: NonNull<ListNode<T>>,
    _phantom: PhantomData<&'list mut ListNode<T>>
}

impl<'list, T> ListWalker<'list, T> {
    #[inline]
    fn new(start_ref: &'list mut ListLink<T>) -> Self {
        let raw = match start_ref.as_deref_mut() {
            Some(node) => node as *mut _,
            None => panic!(),
        };

        Self {
            curr_ptr: NonNull::new(raw).unwrap(),
            _phantom: PhantomData,
        }
    }

    #[inline]
    fn move_forward(&mut self) -> Result<(), ()> {
        let curr_node = unsafe { self.curr_ptr.as_mut() };

        match curr_node.next.as_deref_mut() {
            Some(next_node) => {
                self.curr_ptr = NonNull::from(next_node);
                Ok(())
            }

            None => Err(()),
        }
    }

    #[inline]
    fn into_inner_ref(mut self) -> &'list mut ListNode<T> {
        unsafe { self.curr_ptr.as_mut() }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn append_front(&mut self, val: T) {
        let new_node = ListNode::new(val, self.head.take()).into();
        let _ = self.head.insert(new_node);
    }

    pub fn append_back(&mut self, val: T) {
        let new_node = ListNode::new(val, None).into();

        let tail = match self.head.as_deref_mut() {
            None => &mut self.head,

            head @ Some(_) => {
                let mut curr_node: Option<&mut ListNode<T>> = head;

                while let Some(ListNode{ next: n @ Some(_), .. }) = curr_node {
                    curr_node = n.as_deref_mut();
                }
        
                &mut curr_node.unwrap().next
            }
        };

        *tail = new_node;
    }

    pub fn walk(&mut self) {
        let mut walker = ListWalker::new(&mut self.head);

        println!("starting walk @ head...");

        while let Ok(()) = walker.move_forward() {
            println!("Walked to another node");
            // println! ("Node here, next pointer {:#}", &node_ref.next as *const _ as usize);
        }
    }

    fn calculate_size(&mut self) -> usize {
        let mut cnt = 1_usize;

        let mut walker = ListWalker::new(&mut self.head);

        while let Ok(()) = walker.move_forward() {
            cnt += 1;
        }

        cnt
    }
}

impl<'list, T> LinkedList<T> {
    fn get_tail_mut(&'list mut self) -> &'list mut ListLink<T> {

        // The crux of the issue, this situation:
        match self.head.as_deref_mut() {
            head @ None => &mut self.head,

            head @ Some(_) => {
                let mut curr_node: Option<&mut ListNode<T>> = self.head.as_deref_mut();

                while let Some(ListNode{ next: n @ Some(_), .. }) = curr_node {
                    curr_node = n.as_deref_mut();
                }
        
                &mut curr_node.unwrap().next
            }
        }

        // let mut curr_node: Option<&mut ListNode<T>> = self.head.as_deref_mut();

        // while let Some(ListNode{ next: n @ Some(_), .. }) = curr_node {
        //     curr_node = n.as_deref_mut();
        // }

        // &mut curr_node.unwrap().next

        // Using unsafe Rust, raw pointers

        // match self.head.as_deref_mut() {
        //     None => &mut self.head,

        //     Some(node) => {
        //         let mut curr_ptr = node as *mut ListNode<T>;

        //         while let Some(n) = unsafe{ curr_ptr.as_mut() }.map(|n| n.next.as_deref_mut()).flatten() {
        //             curr_ptr = n as *mut ListNode<T>;
        //         }
        
        //         unsafe{ &mut curr_ptr.as_mut().unwrap().next }
        //     }
        // }



        // Using Polonius, a new take on borrow checker to bypass NLL false-positives
        // let mut curr_node: &mut ListLink<T> = &mut self.head;

        // polonius_loop!(
        //     |curr_node| -> &'polonius mut ListLink<T> {
        //         match curr_node.as_deref_mut() {
        //             None => polonius_return!(),

        //             Some(n) => {
        //                 curr_node = &mut n.next;
        //                 polonius_continue!();
        //             }
        //         }
        //     }
        // );
        
        // curr_node
        

        // using recursive_reference
        // let mut recref = RecRef::new(&mut self.head);

        // while let Ok(()) = RecRef::map_result(&mut recref, |node| match node.as_deref_mut() {
        //     Some(ListNode{ next: n @ Some(_), .. }) => Ok(n),
        //     _ => Err(()),
        // }) {}

        // RecRef::into_ref(recref)


        // Using my ListWalker, a wrapping around unsafe code
        // let mut walker = ListWalker::new(&mut self.head);

        // while let Ok(()) = walker.move_forward() {}

        // &mut walker.into_inner_ref().next


    }
}

pub struct Iter<'list, T> {
    curr_node: Option<&'list ListNode<T>>,
}

impl<'list, T> LinkedList<T> {
    pub fn iter(&'list self) -> Iter<'list, T> {
        Iter {
            curr_node: self.head.as_deref(),
        }
    }
}

impl<'list, T> Iterator for Iter<'list, T> {
    type Item = &'list T;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr_node.map(|node| {
            self.curr_node = node.next.as_deref();
            &node.val
        })
    }
}
fn main() {
    let mut ll = LinkedList::new();
    // ll.append_front(1);
    ll.append_back(2);
    ll.append_front(3);
    ll.append_back(4);
    // ll.append_back(5);
    // ll.append_back(7);

    ll.walk();

    println!("size is {}", ll.calculate_size());

    for (idx, val) in ll.iter().enumerate() {
        println!("element {} - {}", idx, *val);
    }
}
