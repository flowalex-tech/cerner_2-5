/*Given a fixed length array arr of integers, duplicate each occurrence of zero, shifting the remaining elements to the right.
Note that elements beyond the length of the original array are not written.
Do the above modifications to the input array in place, do not return anything from your function.
Example 1:
Input: [1,0,2,3,0,4,5,0]
Output: null
Explanation: After calling your function, the input array is modified to: [1,0,0,2,3,0,0,4]
Example 2:
Input: [1,2,3]
Output: null
Explanation: After calling your function, the input array is modified to: [1,2,3]
Note:
1 <= arr.length <= 10000
0 <= arr[i] <= 9
https://leetcode.com/problems/duplicate-zeros/ */
// cerner_2^5_2021

fn main() {
    let mut arr = vec![1,0,2,3,0,4,5,0];
    //let arr = vec![1, 2, 3]
    //let arr = vec![0, 8, 2]
    for i in arr {
        if arr[i] == 0 {
            arr.insert(i,0);
            arr.pop();
        }
    }
    for x in 0..arr.len() {
        println!("{}", arr[x]);
    }
}
