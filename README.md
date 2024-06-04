### Example
```rust
use typeon::TypeInfo;

#[derive(TypeInfo)]
pub struct Book {
    //...
}

fn main() {
    let book = Book { 
        // ...
    };
    
    assert_eq!("book", Book::TYPE_NAME);
    assert_eq!("book", book.type_name());
}
```