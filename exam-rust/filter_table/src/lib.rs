#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Self {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(Vec::from(row));
    }

    pub fn filter_col<T>(&self, filter: T) -> Option<Self>
    where
        T: Fn(&str) -> bool,
    {
        let mut table = Table::new();
        let col_to_filter = self.headers.iter().map(|v| v.as_str()).position(filter);
        

        match col_to_filter {
            Some(value) => {
                table.headers = vec![self.headers.clone()[value].clone()];
             
                for row_step in 0..self.body.len() {
                    let value = &self.body.clone()[row_step][value];
                    table.add_row(&[value.to_string()]);
                }

                Some(table)
            }
            None => None,
        }
    }

    pub fn filter_row<T>(&self, col_name: &str, filter: T) -> Option<Self>
    where
        T: Fn(&str) -> bool,
    {
        let mut table = Table::new();
        table.headers = self.headers.clone();

        let col_index = self.headers.iter().position(|header| header == col_name);
        match col_index {
            Some(index) => {
                for row in &self.body {
                    if let Some(value) = row.get(index) {
                        if filter(value) {
                            table.add_row(row);
                        }
                    }
                }

                Some(table)
            }
            None => None,
        }
    }
}
