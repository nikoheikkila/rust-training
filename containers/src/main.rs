use crate::containers::container::Container;

pub mod containers;

fn main() {
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let robert = "Robert".to_string();

    let mut container = Container::new();
    container.insert(bob.clone(), robert.clone());

    assert_eq!(container.get(&alice), None);
    assert_eq!(container.get(&bob), Some(&robert));

    println!("{} -> {:?}", &alice, container.get(&alice));
    println!("{} -> {:?}", &bob, container.get(&bob));
}
