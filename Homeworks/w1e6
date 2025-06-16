fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::with_capacity(nums.len());
    let mut s = 0;
        
    for item in nums {
        s += item;
        new_vec.push(s);
    }
    new_vec
}


fn main() {
    let example1 = running_sum(vec![1, 2, 3, 4]);
    let example2 = running_sum(vec![1, 1, 1, 1, 1, 1]);
    let example3 = running_sum(vec![3, 1, 2, 10, 1]);
    
    println!("Example 1: {:?} \nExample 2: {:?} \nExample 3: {:?}", example1, example2, example3);
}
