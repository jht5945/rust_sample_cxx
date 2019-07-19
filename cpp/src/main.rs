#[macro_use]
extern crate cpp;

cpp!{{
    #include <stdio.h>
    #include <iostream>
}}

fn main() {
    unsafe {
        cpp!([] {
            printf("Hello, World!\n");
            std::cout << "Hello, World2!" << std::endl;
        });
    }
}
