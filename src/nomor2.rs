fn receive(param1: &str, param2: &str){

    let param1= param1.to_string();
    let param1 = param1.parse::<i32>().unwrap();
    println!("param1 = {}", param1);

    let param2= param2.to_string();
    let param2 = param2.parse::<i32>().unwrap();
    println!("param2 = {}", param2);

    println!("");
    println!("output = {}", param1 % param2)
}

fn main() {
    receive("20", "7");
}