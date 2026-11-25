// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut res = 42;
    let option = Some(12);
    // clippy::option_map_unit_fn / for_loop_over_option: an Option holds at
    // most one item, so iterating it reads worse than destructuring it.
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
