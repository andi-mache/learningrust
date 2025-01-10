fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6];

    // 1.iter immutable
    ////
    //for num in nums.iter() {
    //    println!("num: {num}")
    //}
    //println!("{:?}", nums); // use nums after iter 
    //

    //for num in nums.iter_mut() {
    //    *num += 1;
    //}
    //println!("{:?}", nums); // use nums after iter
    ////
    //
   for num in nums.into_iter() {
        println!("num: {num}")
    }
    //println!("{:?}", nums); // use nums after iter

}
