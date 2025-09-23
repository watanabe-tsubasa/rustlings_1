macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

macro_rules! second_macro {
    () => {
        println!("made by original.");
    };
}

fn main() {
    // TODO: Fix the macro call.
    my_macro!();
    second_macro!();
}
