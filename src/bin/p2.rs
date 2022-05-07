macro_rules! my_test_macro {
    ($val: expr) => {
        let x = $val;
        println!("{}", x);
    };
}

fn main() {
    my_test_macro!("hello world");
}
