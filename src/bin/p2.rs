// Topic: Try test macro basic implementation
//
// fragment specifiers =  expr, ident, path, ty, item, tt, block and so on
// description = expression, identifier, path, type, top level item, token-tree, block and so on
// Examples:
// expr = 3,x, func(a+2), if c{1} else {2}, loop {}
// ident = x, main, my_test_macro
// path = x, main, module::Type, ::crate::Type, x::<T>::y, <T>::foo
// item = extern "C", fn foo(), use std::fs;, struct Test;
// tt= fn, main, (), (3)
// {},{3},{let x = 4}

macro_rules! my_test_macro {
    ($val: expr) => {
        let x = $val;
        println!("{}", x);
    };
}

fn main() {
    my_test_macro!("hello world");
}
