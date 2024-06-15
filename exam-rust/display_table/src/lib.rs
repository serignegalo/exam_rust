
use std::fmt;

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

    fn get_column_widths(&self) -> Vec<usize> {
        let mut widths = vec![0; self.headers.len()];

        for (i, header) in self.headers.iter().enumerate() {
            widths[i] = header.len();
        }

        for row in &self.body {
            for (i, cell) in row.iter().enumerate() {
                if cell.len() > widths[i] {
                    widths[i] = cell.len();
                }
            }
        }

        widths
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.headers.is_empty() {
            return Ok(());
        }

        let widths = self.get_column_widths();

        let draw_separator = |f: &mut fmt::Formatter<'_>| -> fmt::Result {
            let mut to_write = String::new();
            for width in &widths {
                to_write.push_str(format!("+{}", "-".repeat(*width + 2)).as_str())
            }
            writeln!(f, "|{}|", to_write.get(1..).unwrap_or_default())
        };

        let draw_row = |f: &mut fmt::Formatter<'_>, row: &[String]| -> fmt::Result {
            let mut to_write = String::new();
            for (cell, width) in row.iter().zip(&widths) {
                let padding = (*width - cell.len()) + 2;

                let mut left_padding = padding / 2;
                let mut right_padding = padding / 2;

                if padding % 2 != 0 {
                    let space = padding - 1;
                    left_padding = space / 2;
                    right_padding = space / 2 + 1;
                } 
                
                to_write.push_str(format!("|{}{}{}", " ".repeat(left_padding), cell, " ".repeat(right_padding)).as_str());
            }

            writeln!(f, "{}|", to_write)
        };

        // Print headers
        draw_row(f, &self.headers)?;

        // Print separator
        draw_separator(f)?;

        // Print rows
        for row in &self.body {
            draw_row(f, row)?;
        }

        Ok(())
    }
}




/* 
#[derive(Debug)]
struct Format {
    pub size: usize,
    pub cols: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

fn test(value: String, size: usize) -> String {
    let mut o = String::new();
    let space = size.checked_sub(value.len()).unwrap_or_default() + 2;
    if space % 2 == 0 {
        o.push_str(
            format!(
                "|{}{}{}",
                " ".repeat(space / 2),
                value,
                " ".repeat(space / 2)
            )
            .as_str(),
        );
    } else {
        let a = space.checked_sub(1).unwrap_or_default() / 2;
        o.push_str(
            format!(
                "|{}{}{}",
                " ".repeat(a),
                value,
                " ".repeat(a + 1)
            )
            .as_str(),
        );
    }
    o
}

use std::fmt;
impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = Format::f(self.clone());
        if s.is_none() {
            return Ok(());
        }

        let s = s.unwrap();
        let mut num_col = 0;
        let mut a: Vec<Vec<String>> = Vec::new();

        for value in s.iter() {
            let mut c: Vec<String> = Vec::new();
            for g in value.cols.iter().enumerate() {
                c.push(test(g.1.clone(), value.size));
                if g.0 == 0 {
                    c.push(format!("{}+", "-".repeat(value.size + 2)));
                }
            }
            a.push(c.clone());
            num_col = c.clone().len();
        }

        for col in 0..num_col {
            let mut to_write = String::new();
            for row in a.iter() {
                to_write.push_str(row[col].as_str());
            }
            if col == 1 {
                to_write.pop();
                to_write = format!("|{}", to_write);
            }

            write!(f, "{}|\n", to_write);
        }

        Ok(())
    }
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
}

impl Format {
    pub fn f(item: Table) -> Option<Vec<Self>> {
        let mut a: Vec<Format> = Vec::new();

        if item.headers.len() == 0 {
            return None;
        }

        for index_col in 0..item.headers.len() {
            let mut b: Format = Format {
                size: item.headers[index_col].len(),
                cols: vec![item.headers[index_col].clone()],
            };

            for row in item.body.iter() {
                let content = row[index_col].clone();
                if content.len() > b.size {
                    b.size = content.len();
                }
                b.cols.push(content);
            }
            a.push(b);
        }
        Some(a)
    }
}
*/