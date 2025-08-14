use std::{cell::RefCell, rc::Rc};

pub struct LinkedList<Item> {
    head: Option<Rc<RefCell<LinkedListNode<Item>>>>,
    tail: Option<Rc<RefCell<LinkedListNode<Item>>>>,
    count: i64,
}

struct LinkedListNode<Item> {
    val: Item,
    next: Option<Rc<RefCell<LinkedListNode<Item>>>>,
}
type ListNodePtr<Item> = Rc<RefCell<LinkedListNode<Item>>>;

pub struct LinkedListIter<'a, Item> {
    cur: Option<ListNodePtr<Item>>,
    _marker: std::marker::PhantomData<&'a Item>,
}

impl<Item> LinkedList<Item> {
    pub fn new() -> LinkedList<Item> {
        LinkedList {
            head: None,
            tail: None,
            count: 0,
        }
    }

    pub fn push(self: &mut Self, v: Item) -> &mut Self {
        let new_node = Rc::new(RefCell::new(LinkedListNode { val: v, next: None }));

        match &self.tail {
            Some(tail) => {
                tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }

        self.count += 1;
        self
    }

    pub fn count(self: &mut Self) -> i64 {
        self.count
    }

    pub fn iter<'a>(&'a self) -> LinkedListIter<'a, Item> {
        LinkedListIter {
            cur: self.head.clone(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, Item> Iterator for LinkedListIter<'a, Item> {
    type Item = &'a Item;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cur.take()?;
        let borrowed = cur.borrow();
        self.cur = borrowed.next.clone();
        let val_ref = unsafe { &*(&borrowed.val as *const Item) };
        Some(val_ref)
    }
}
