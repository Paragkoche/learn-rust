mod basic {
    pub mod array;
    pub mod arraysorting;
    pub mod datatype;
    pub mod hello_wold;
    pub mod loops;
}

// use basic::hello_wold
fn main() {
    // basic::hello_wold::hello_world();
    // basic::datatype::datatype();
    // basic::loops::loops();
    // basic::array::arrays();
    let mut arry: [i32; 10] = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let sArry = basic::arraysorting::sorting_array(&mut arry);
    for i in sArry {
        println!("{i}")
    }
}
