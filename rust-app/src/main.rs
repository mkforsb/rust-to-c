mod bindings;

use crate::bindings::Person;

fn main() {
    let x = Person::new("Pelle Rambosson".to_string(), 330303);
    let _ = dbg!(x.fmt());
}
