//! Demo of Rust and axum web framework.
//!
//! <https://github.com/joelparkerhenderson/demo-rust-axum>
//!
//! For more see the file `README.md` in the project root.
//!
//! This example uses a `Book` struct, a `DATA` variable
//! that is a lazy RwLock global variable, and handlers
//! that process the routes for HTTP verbs GET, PUT, etc.

use std::thread;

use axum::{response::IntoResponse, routing::get};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

pub fn app() -> axum::Router {
    axum::Router::new()
        .route("/books", get(get_books).post(post_books))
        .route(
            "/books/{id}",
            get(get_books_id)
                .put(put_books_id)
                .patch(patch_books_id)
                .delete(delete_books_id),
        )
}

mod book;
use crate::book::{Book, BookCreate};

mod book_change;
use crate::book_change::BookChange;

mod data;
use crate::data::DATA;

pub async fn get_books() -> axum::response::Response {
    thread::spawn(move || {
        if let Ok(data) = DATA.read() {
            let mut books = data.values().collect::<Vec<_>>().to_owned();
            books.sort_by(|a, b| a.title.cmp(&b.title));
            (axum::http::StatusCode::OK, axum::response::Json(books)).into_response()
        } else {
            axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    })
    .join()
    .unwrap()
}

pub async fn post_books(axum::extract::Json(book): axum::extract::Json<BookCreate>) -> axum::http::StatusCode {
    thread::spawn(move || {
        if let Ok(mut data) = DATA.write() {
            let id = data.keys().max().unwrap() + 1;
            let book = Book {
                id,
                title: book.title,
                author: book.author,
            };
            data.insert(id, book);
            axum::http::StatusCode::CREATED
        } else {
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        }
    })
    .join()
    .unwrap()
}

pub async fn get_books_id(axum::extract::Path(id): axum::extract::Path<u32>) -> axum::response::Response {
    thread::spawn(move || {
        if let Ok(data) = DATA.read() {
            match data.get(&id) {
                Some(book) => (axum::http::StatusCode::OK, axum::response::Json(book)).into_response(),
                None => axum::http::StatusCode::NOT_FOUND.into_response(),
            }
        } else {
            axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    })
    .join()
    .unwrap()
}

pub async fn put_books_id(
    axum::extract::Path(id): axum::extract::Path<u32>,
    axum::extract::Json(book): axum::extract::Json<BookCreate>,
) -> axum::http::StatusCode {
    thread::spawn(move || {
        if let Ok(mut data) = DATA.write() {
            let book = Book {
                id,
                title: book.title,
                author: book.author,
            };
            data.insert(id, book);
            axum::http::StatusCode::CREATED
        } else {
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        }
    })
    .join()
    .unwrap()
}

pub async fn delete_books_id(axum::extract::Path(id): axum::extract::Path<u32>) -> axum::http::StatusCode {
    thread::spawn(move || {
        if let Ok(mut data) = DATA.write() {
            if data.remove(&id).is_some() {
                axum::http::StatusCode::NO_CONTENT
            } else {
                axum::http::StatusCode::NOT_FOUND
            }
        } else {
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        }
    })
    .join()
    .unwrap()
}

pub async fn patch_books_id(
    axum::extract::Json(book_change): axum::extract::Json<BookChange>,
) -> axum::http::StatusCode {
    thread::spawn(move || {
        let id = book_change.id;
        if let Ok(mut data) = DATA.write() {
            if let Some(book) = data.get_mut(&id) {
                if let Some(title) = book_change.title {
                    book.title = title;
                }
                if let Some(author) = book_change.author {
                    book.author = author;
                }
                axum::http::StatusCode::NO_CONTENT
            } else {
                axum::http::StatusCode::NOT_FOUND
            }
        } else {
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        }
    })
    .join()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::reset;
    use axum_test::TestServer;
    use serde_json::json;

    fn setup() -> TestServer {
        reset();
        TestServer::new(app()).unwrap()
    }

    #[tokio::test]
    async fn get_books() {
        let server = setup();
        server.get("/books").await.assert_json(&json!([
            { "id": 1, "title": "Antigone", "author": "Sophocles" },
            { "id": 2, "title": "Beloved", "author": "Toni Morrison" },
            { "id": 3, "title": "Candide", "author": "Voltaire" }
        ]));
    }

    #[tokio::test]
    async fn post_books() {
        let server = setup();
        let j = json!({ "title": "Decameron", "author": "Giovanni Boccaccio" });
        server
            .post("/books")
            .json(&j)
            .await
            .assert_status(axum::http::StatusCode::CREATED);
    }

    #[tokio::test]
    async fn get_books_id() {
        let server = setup();
        server.get("/books/1").await.assert_json(&json!(
            { "id": 1, "title": "Antigone", "author": "Sophocles" }
        ));
    }

    #[tokio::test]
    async fn put_books_id() {
        let server = setup();
        let j = json!({ "title": "Decameron", "author": "Giovanni Boccaccio" });
        server
            .put("/books/4")
            .json(&j)
            .await
            .assert_status(axum::http::StatusCode::CREATED);
    }

    #[tokio::test]
    async fn patch_books_id() {
        let server = setup();
        let j = json!({ "id": 1, "title": "Elektra" });
        server
            .patch("/books/1")
            .json(&j)
            .await
            .assert_status(axum::http::StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn delete_books_id() {
        let server = setup();
        server
            .delete("/books/1")
            .await
            .assert_status(axum::http::StatusCode::NO_CONTENT);
    }
}
