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

    
    // async { 
    //     let p = 5;
    //     let future = unsafe_example(&p);
    //     future.await;
    // }

    let s = MyStruct::from("hello");
    println!("{:#?}", s);

    // let p = MyStruct { x: 77 };
    let p: MyStruct = "heelo".into();
    println!("{:#?}", p);

    
}

async fn unsafe_example(x: &u32) {
    println!{"{}", x};
}

#[derive(Debug)]
struct MyStruct {
    x: u32,
}

impl From<&str> for MyStruct {
    #[inline]
    fn from(_: &str) -> MyStruct {
        MyStruct { x: 42 }
    }
}