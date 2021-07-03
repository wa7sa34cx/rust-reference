macro_rules! example {
    () => { println!("Macro call in a macro!") };
}

macro_rules! plus {
    ($t:tt, $v:tt) => { $t + $v; };
}

fn main() {
    example!();
    println!();

    let x = plus!(5.0, 7.2);
    println!("{}", x);
}
