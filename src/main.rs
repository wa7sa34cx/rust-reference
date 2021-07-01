macro_rules! example {
    () => { println!("Macro call in a macro!") };
}

fn main() {
    example!();
}
