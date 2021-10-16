fn majority_element(mut nums: Vec<i32>) -> i32{
    nums.sort();
    let mut count: i32 = 0;
    for i in 0..nums.len() {
        if (i as i32) - 1 == - 1 || nums[i - 1] == nums[i] {
            count += 1;
        } else {
            count = 1
        }
        if count > (nums.len() / 2) as i32 {
            return nums[i];
        }
    }
    -1
}
fn main() {
    let result = majority_element(Vec::from([3, 2, 3]));
    println!("{:?}", result);
    let result = majority_element(Vec::from([2,2,1,1,1,2,2]));
    println!("{:?}", result);
}
// cerner_2tothe5th_2021
