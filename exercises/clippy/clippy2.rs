// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

// I AM  DONE

fn main() {
    let mut res = 42;
    let option = Some(12);
    // if let Some(x) = option {
    //     Some(x)
        
    // }
    if let Some(x)=option {
        res += x;

    } 
    // x in option {
    // }
    println!("{}", res);
}
