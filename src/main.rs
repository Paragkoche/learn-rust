mod basic {
    pub mod array;
    pub mod datatype;
    pub mod hello_wold;
    pub mod loops;
}

// use basic::hello_wold
fn main() {
    basic::hello_wold::hello_world();
    basic::datatype::datatype();
    basic::loops::loops();
    basic::array::arrays();
}
