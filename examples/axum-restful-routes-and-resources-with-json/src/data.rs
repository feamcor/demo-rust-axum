use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

use crate::book::Book;

fn create_initial_data() -> HashMap<u32, Book> {
    HashMap::from([
        (
            1,
            Book {
                id: 1,
                title: "Antigone".into(),
                author: "Sophocles".into(),
            },
        ),
        (
            2,
            Book {
                id: 2,
                title: "Beloved".into(),
                author: "Toni Morrison".into(),
            },
        ),
        (
            3,
            Book {
                id: 3,
                title: "Candide".into(),
                author: "Voltaire".into(),
            },
        ),
    ])
}

pub static DATA: LazyLock<RwLock<HashMap<u32, Book>>> = LazyLock::new(|| RwLock::new(create_initial_data()));

#[cfg(test)]
pub fn reset() {
    if let Ok(mut data) = DATA.write() {
        *data = create_initial_data();
    }
}
