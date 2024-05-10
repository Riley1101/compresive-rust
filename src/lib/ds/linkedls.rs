#[derive(Debug)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

#[derive(Debug)]
pub enum ListI32 {
    Empty,
    ElemThenEmpty(i32),
    ElemThenNotEmpty(i32, Box<ListI32>),
}

pub struct Node<T> {
    elem: T,
    next: ListNode<T>,
}

pub enum ListNode<T> {
    Empty,
    More(Box<Node<T>>),
}
