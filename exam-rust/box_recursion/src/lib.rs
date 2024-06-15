/**
 * Using the given code, create the following **associated functions**:

- `new`: which will initialize the `WorkEnvironment` with `grade` set to `None`.
- `add_worker`: which receives two strings, one being the role and the other the name of the worker. It will add the worker at the start of the list.
- `remove_worker`: which removes the last worker that was placed in the `WorkEnvironment`, this function returns an `Option` with the name of the worker.
- `last_worker`: which returns an `Option` with a tuple containing the name and role of the last added worker.

You must also create a type named `Link`. This will be the connection between the `WorkEnvironment` and `Worker` structures. This will be a recursion type, and it must point to `None` if there is no `Worker` to point to.
 */

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        Self { grade: None }
    }

    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Box::new(Worker {
            name,
            role,
            next: self.grade.take(),
        });
        self.grade = Some(new_worker);
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        match self.grade.take() {
            Some(value) => {
                self.grade = value.next;
                Some(value.name)
            }
            None => Some(String::new()),
        }
    }

    pub fn last_worker(&self) -> Option<(String, String)> {
        match &self.grade {
            Some(value) => Some((value.name.to_string(), value.role.to_string())),
            None => Some((String::new(), String::new())),
        }
    }
}
