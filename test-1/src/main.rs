macro_rules! generate_fucntions {
    ($($name:ident),*) => {
        $(
            fn $name() {
                println!("Function {:?} called", stringify!($name));
            }
        )*
    };
}


fn main() {
    generate_fucntions!(foo, bar, baz);
    foo();
    bar();
    baz();
}





