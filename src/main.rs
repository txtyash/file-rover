use std::env;

fn main() {
    let selection: Vec<String> = env::args().skip(1).collect();
    if selection.len() < 2 {
        let output: Vec<char> = selection[0].chars().collect();
        println!("{:?}", output);
    } else {
        println!("{:?}", selection);
    }
}
