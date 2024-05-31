pub use library::writers::Writer;
pub use library::books::Book;

pub mod library {
    pub mod writers {
        use super::books::Book;
        pub struct Writer {
            pub first_name: String,
            pub last_name: String,
            pub books: Vec<Book>
        }
    }
    pub mod books{
        #[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
        pub struct Book {
            pub title: String,
            pub year: u64
        }
    }
}

pub fn order_books(writer: &mut Writer) {
    writer.books.sort();
}
