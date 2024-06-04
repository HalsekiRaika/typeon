use typeon::TypeInfo as _;
use typeon_derive::TypeInfo;

#[derive(TypeInfo)]
pub struct Book {}

#[test]
fn derive() {
    let book = Book {};
    assert_eq!("book", Book::TYPE_NAME);
    assert_eq!("book", book.type_name());
}