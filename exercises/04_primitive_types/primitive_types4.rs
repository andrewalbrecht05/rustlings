// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.


fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    println!("Розмір масиву а: {}", a.len());
    println!("Масив займає {} байт", std::mem::size_of_val(&a));
}
