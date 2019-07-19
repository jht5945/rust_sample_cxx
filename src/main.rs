#![plugin(rustcxx_plugin)]

cxx_inline! {
    #include <stdint.h>
    uint32_t square_it(uint32_t x) {
        return rust![(x: u32) -> u32 {
            println!("Rust: Squaring {}", x);
            x * x
        }];
    }
}

let x: u32 = 5;
let square = unsafe { cxx![(x: u32) -> u32 {
    std::cout << "C++: Squaring " << x << std::endl;
    square_it(x)
}] };
