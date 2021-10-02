//Optional Tests: https://gist.github.com/flowalex-tech/2c9279c7fb6464a067718715558d73b7
// cerner_2tothe5th_2021
fn nonogram(v: &[i8]) -> Vec<usize> {
    let mut ans = Vec::<usize>::new();
    let mut count = 0;
    for i in v {
        if *i == 1 {
            count += 1;
        } else if count != 0 {
            ans.push(count);
            count = 0;
        }
    }
    if count != 0 {
        ans.push(count);
    }
    ans
}
fn main() {
    let result = nonogram(&[0, 0, 0, 0, 0]);
    println!("{:?", result)
    let result = nonogram(&[1, 1, 1, 1, 1]);
    println!("{:?", result)
    let result = nonogram(&[0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1]);
    println!("{:?", result)
    let result = nonogram(&[1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0]);
    println!("{:?", result)
    let result = nonogram(&[0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1]);
    println!("{:?", result)
    let result = nonogram(&[1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]);
    println!("{:?", result)
}
