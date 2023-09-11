#![allow(dead_code)]

struct Person {
  name: String,
  age: u32
}

struct Book {
  name: String,
  author: String, 
  is_available: bool
}

struct Library {
  books: Vec<Book>
}