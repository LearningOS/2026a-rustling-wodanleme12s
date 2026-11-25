// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    // push() below hands over a &'static str, so that is the element type.
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
