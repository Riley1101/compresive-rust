#[derive(Debug)]
pub struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn from(value: i32) -> Self {
        Node { value, next: None }
    }
}

pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new(value: i32) -> Self {
        Self {
            head: Some(Box::new(Node::from(value))),
        }
    }

    fn push(&mut self, value: i32) {
        match self.head.take() {
            Some(node) => {
                let new_node = Node {
                    value,
                    next: Some(node),
                };
                self.head = Some(Box::new(new_node));
            }
            None => self.head = Some(Box::new(Node { value, next: None })),
        }
    }

    fn count(self) -> i32 {
        let mut count = 0;
        let mut cur = self.head;
        while cur.is_some() {
            println!("{:?}", cur);
            count += 1;
            cur = cur.unwrap().next;
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_list() {
        let mut list = LinkedList::new(0);
        for i in 1..12 {
            list.push(i);
        }
        assert_eq!(list.count(), 12);
    }
}
