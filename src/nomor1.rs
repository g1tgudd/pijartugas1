fn to_string(param1: &str, param2: &str) -> String {

    let param1 = String::from(param1);
    let param2 = String::from(param2);
    println!("param1 = {:?}", param1);
    println!("param2 = {:?}", param2);

    let param3 = String::from(param1 + " " + &param2);

    println!("");

    return (param3);
}

fn main() {

    let output = to_string("I love", "Rust");
    println!("output = {:?}", output);

}