use std::thread;

fn main() {
    example_move_into_closure();
    example_mut_borrow_closure();
    exemple_immutable_borrow_closure();
}

fn example_move_into_closure() {
    println!("\n---\nEXAMPLE: Move into closure\n---");
    let list = vec![1, 2, 3];

    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("In thread: {:?}", list))
        .join()
        .unwrap();

    // Can not use list here as it's moved into the closure of the thread above when we use the
    // keyword 'move' to force it into the closure as an owned value.
    // println!("{:?}", list);
}

fn example_mut_borrow_closure() {
    println!("\n---\nEXAMPLE: Mutable borrow with closure\n---");
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrow_mutably = || list.push(8);

    // Can not print this here as list is borrowed as mutable in the closure above already
    // And if a value is mutably borrowed we need to get it back before we can refernce it again.
    // println!("After calling closure: {:?}", list);

    borrow_mutably();

    println!("After calling closure: {:?}", list);
}

fn exemple_immutable_borrow_closure() {
    println!("\n---\nEXAMPLE: Immutable borrow with closure\n---");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // Borrow as a immutable reference only.

    let only_borrow = || println!("In closure: {:?}", list);

    // Then we can use the `list` value below as
    // it isn't moved into the closure

    println!("Before calling closure: {:?}", list);

    only_borrow();

    println!("After calling closure: {:?}", list);
}
