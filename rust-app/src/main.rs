mod bindings;

use crate::bindings::Person;

fn main() {
    let mut x = Person::new("Pelle Rambosson".to_string(), 330303);
    x.set_age(42);
    println!("{}", x.get_age());
    let _ = dbg!(x.fmt());
}
