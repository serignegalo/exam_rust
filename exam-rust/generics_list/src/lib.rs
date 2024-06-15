#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
    pub len: usize,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List{head: None, len: 0}
    }

    pub fn push(&mut self, value: T) {
        self.len += 1;
        let new_node = Node{
            value,
            next: self.head.take().map(Box::new),
        };
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) {
        self.len -= 1;
        if let Some(value) = self.head.take() {
            self.head = value.next.map(|x| *x);
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}