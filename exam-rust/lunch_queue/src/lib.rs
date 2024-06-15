#[derive(Debug, Clone)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;

#[derive(Debug, Clone)]
pub struct Person {
    pub discount: i32,
    pub name: String,
    pub next_person: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Self { node: None }
    }

    pub fn add(&mut self, name: String, discount: i32) {
        let new_node = Some(Box::new(Person {
            name,
            discount,
            next_person: self.node.take(),
        }));

        self.node = new_node;
    }

    pub fn invert_queue(&mut self) {
        let mut a: Vec<Person> = Vec::new();
        let mut current = self.node.take();

        while let Some(person) = current.clone() {
            let p = Person {
                name: person.name,
                discount: person.discount,
                next_person: None,
            };
            a.push(p);
            current = person.next_person;
        }

        let mut new_node = Self::new();
        for person in a {
            new_node.add(person.name, person.discount)
        }

        self.node = new_node.node;
      
    }

    pub fn rm(&mut self) -> Option<(String, i32)> {
        let mut a: Vec<Person> = Vec::new();
        let mut current = self.node.take();

        while let Some(person) = current.clone() {
            let p = Person {
                name: person.name,
                discount: person.discount,
                next_person: None,
            };
            a.push(p);
            current = person.next_person;
        }

        let removed_person = a.pop();
        let mut new_node = Self::new();
        for person in a.iter().rev() {
            new_node.add(person.name.clone(), person.discount)
        }

        self.node = new_node.node;

        match removed_person {
            Some(person) => Some((person.name, person.discount)),
            None => None,
        }
    }

    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        let mut p = self.node.clone();
        while p.is_some() {
            let a: Box<Person> = p.unwrap();
            if a.name.clone() == name {
                return Some((a.name.clone(), a.discount.clone()));
            };
            p = a.next_person;
        }

        None
    }
}
