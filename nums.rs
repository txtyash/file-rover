use std::env;
fn main() {
    let args: Vec<i32> = env::args()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    //let num_array: Vec<i32> = args.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    println!("{:?}", args);
}
