pub fn arrays() {
    let mut arry: [i32; 10] = [0; 10]; // Initialize the array with all elements set to 0
    for l in 0..10 {
        println!("Enter the value of arry[{l}]: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input);
        arry[l] = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
    }
    let mut i = 0;
    for l in arry {
        println!("arry[{i}] = {l}");
        i += 1; // Print each element of the array
    }
}
