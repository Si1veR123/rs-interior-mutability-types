extern crate interior_mutability;

use interior_mutability::cell::Cell;


fn main() {
    let cell = Cell::new(5);
    println!("{}", cell.replace(6));
    println!("{}", cell.replace(10));
    println!("{}", cell.get());
    cell.set(100);
    println!("{}", cell.get());
    cell.set(1000);
    println!("{}", cell.into_inner());
}
