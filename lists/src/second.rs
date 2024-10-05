#![allow(dead_code)]

use std::mem;

#[derive(Debug)]
pub struct Node {
    elem: i32,
    next: Link,
}

type Link = Option<Box<Node>>;

#[derive(Debug)]
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, ele: i32) {
        let new_node = Box::new(Node {
            elem: ele,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // match self.head.take() {
        //     Some(node) => {
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        //     None => None,
        // };
        //
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut curr_link = self.head.take();
        while let Some(mut box_node) = curr_link {
            curr_link = box_node.next.take();
        }
    }
}
