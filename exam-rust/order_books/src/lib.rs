pub mod library {
    pub mod writers {
        use super::books;

        #[derive(Debug, Clone, PartialEq)]
        pub struct Writer {
            pub first_name: String,
            pub last_name: String,
            pub books: Vec<books::Book>
        }
    }

    pub mod books {

        #[derive(Debug, Clone, PartialEq)]
        pub struct Book {
            pub title: String,
            pub year: u64
        }
    }
}

#[allow(dead_code)]
pub fn order_books(writer: &mut library::writers::Writer) {
    writer.books.sort_by(|a, b| a.title.cmp(&b.title));
}