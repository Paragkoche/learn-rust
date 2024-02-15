pub fn sorting_array(arry: &mut [i32; 10]) -> &mut [i32; 10] {
    let cont = arry.len();
    // let mut j = 0;
    for i in 0..cont {
        for j in 0..cont {
            if arry[i] < arry[j] {
                let temp = arry[i];
                arry[i] = arry[j];
                arry[j] = temp;
            }
        }
    }
    return arry;
}
