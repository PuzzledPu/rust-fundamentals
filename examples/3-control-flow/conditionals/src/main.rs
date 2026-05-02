fn main() {
    let proceed = true;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not proceeding");
        return; // use std::process::exit(0/1); to exit deep in codebase without cleanup.
    }

    let height = 190;
    if height >= 180 {
        println!("Tall");
    } else if height > 170 {
        println!("Average");
    } else {
        println!("Short");
    }

}
