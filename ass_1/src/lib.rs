use std::fmt;

// ==== PERSON ====

pub struct Person {
  name: String,
  age: u32
}

impl Person {
  pub fn new(name: String, age: u32) -> Person {
    Person {
      name,
      age
    }
  }
}

impl fmt::Display for Person {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} - {} years old", self.name, self.age)
  }
}

// ==== BOOK ====
pub struct Book<'a> {
  id: u32,
  name: String,
  author: String, 
  is_available: bool,
  borrower: Option<&'a Person>
}

// ==== LIBRARY ====
pub struct Library<'a> {
  books: Vec<Book<'a>>
}


impl<'a> Library<'a> {
  pub fn new(books_csv: &str)->Library {
    let mut books:Vec<Book> = vec![];
    let mut current_id = 1;
    for line in books_csv.lines() {
        let mut split = line.split(",");
        
        let book = Book {
            id: current_id,
            name:  split.nth(0).unwrap_or("").to_string(),
            author: split.nth(1).unwrap_or("").to_string(),
            borrower: None, 
            is_available: false
        };

        books.push(book);
        current_id += 1;
    }

    let lib = Library {
        books
    };

    lib
  }
  pub fn checkout(&mut self, id: u32, person: &'a Person){
    for book in self.books.iter_mut(){
        if book.id == id {
            book.is_available = false;
            book.borrower = Some(person);
            return;
        }
    };
  }
  pub fn return_book(&mut self, id: u32){
    for book in self.books.iter_mut(){
        if book.id == id {
            book.is_available = true;
            book.borrower = None;
            return;
        }
    };
  }
  pub fn list_books(&self){
    for book in self.books.iter(){
        println!("{} - {}", book.name, book.author);
    };
  }
  pub fn list_borrowed(&self){
    for book in self.books.iter(){
        if !book.is_available {
            match book.borrower {
                Some(borrower) => println!("\"{}\" is borrowed by {}", book.name, borrower),
                None => continue
            };
        }
    };
  }
}
