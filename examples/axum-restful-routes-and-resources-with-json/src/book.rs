use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
}

#[derive(Debug, Deserialize)]
pub struct BookCreate {
    pub title: String,
    pub author: String,
}

impl From<BookCreate> for Book {
    fn from(val: BookCreate) -> Self {
        Book {
            id: 0,
            title: val.title,
            author: val.author,
        }
    }
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} by {}", &self.title, &self.author,)
    }
}
