use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        panic!("The line is empty");
    } 
    else if args.len() == 1 {
        let char_array: Vec<char> = args[0].chars().collect();
        println!("{:?}", char_array);
    } else {
        println!("{:?}", args);
    }
}
