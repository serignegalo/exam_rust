#[derive(Debug)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;

#[derive(Debug)]
pub struct Person {
    pub discount: i32,
    pub name: String,
    pub next: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            node: None
        }
    }

    pub fn add(&mut self, name: String, discount: i32) {
        let lolo = Person {
            discount,
            name,
            next: self.node.take()
        };
        self.node = Some(Box::new(lolo));
    }

    pub fn invert_queue(&mut self) {
        let mut my_vec = Vec::new();
        
        let mut current = self.node.take();
        while let Some(mut person) = current {
            current = person.next.take();
            my_vec.push(person);
        }

        let mut new_node = Queue {node: None};
        for person in my_vec.iter() {
            new_node.add(person.name.clone(), person.discount)
        }

        self.node = new_node.node;
    }

    pub fn rm(&mut self) -> Option<(String, i32)> {
        self.invert_queue();
        
        match self.node.take() {
            Some(val) => {
                self.node = val.next;
                self.invert_queue();
                Some((val.name.clone(), val.discount))
            },
            None =>  Some(("".to_string(), 1)),
        }
    }

    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        let mut current = self.node.as_ref();
        while let Some(person) = current {
            if person.name == name {
                return Some((person.name.clone(), person.discount));
            } 
            current = person.next.as_ref();
        }
        None
    }
}//done
