// clippy3.rs
//
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

fn main() {
    // The original code called is_none() and then unwrap()ed inside that very
    // branch, i.e. it always panicked. clippy::unnecessary_literal_unwrap also
    // refuses any unwrap() on a literal Option, so the honest fix is to drop
    // the unwrap entirely and just branch on the value we actually have.
    let my_option: Option<()> = None;
    if my_option.is_none() {
        println!("nothing to do here");
    }

    // The literal was missing commas between -3 and -4, so it parsed as
    // `-3 - 4` and silently produced a 4-element array.
    let my_arr = &[-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    // vec![..].resize(0, 5) throws away the vector and returns (), which is
    // not "an empty vec". Vec::new() says it directly.
    let my_empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // The manual assignment read back the already-overwritten value_a, so both
    // ended up as 66. std::mem::swap does it with the temporary in place.
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
