fn example_of_immutable_versus_mutable() {
    let mut a = ['a', 'b', 'c'];

    let b = &a; // Borrow data immutably
    //b[0] = 'x'; // Changing data won't compile
    println!("{:?}", b[0]); // Reading data is fine

    let c = &mut a; // Borrow data mutably
    c[0] = 'x'; // Changing data is fine.
    println!("{:?}", c[0]); // Reading data is fine.
}

fn example_of_serial_mutable_versus_parallel_mutable() {
    let mut a = ['a', 'b', 'c'];

    // The borrow checker accepts this code because
    // there is only one mutable borrow at a time.
    let b = &mut a;
    b[0] = 'x';
    let c = &mut a;
    c[0] = 'y';

    // The borrow checker rejects this code because
    // there are two mutable borrows that overlap.
    // let b = &mut a;
    // let c = &mut a;
    // b[0] = 'x';
    // c[0] = 'y';
}

fn main() {
    example_of_immutable_versus_mutable();
    example_of_serial_mutable_versus_parallel_mutable();
}

