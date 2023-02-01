use std::io;


fn main() {
    println!("This program calls different functions.");
    try_adding();
}

fn try_adding() {
    println!("* Adding *");
    let (a, my_status) = get_int_from_user("Please enter value A.");
    println!("Entered for A: {}", a);
    let (b, my_status) = get_int_from_user("Please enter value B.");
    println!("Entered for B: {}", b);
    let my_sum = a+b;
    println!("{} + {} = {}", a, b, my_sum);
}

fn get_int_from_user(prompt: &str) -> (i32, NumberParseResult) {
    let mut my_input = String::new();

    println!("{}", prompt);
    io::stdin()
        .read_line(&mut my_input)
        .expect("Failed to read line");

    let mut my_status: NumberParseResult = NumberParseResult::ok;
    let my_num: i32 = match my_input.trim().parse()
    {
        Ok(num) => num,
        Err(_) => {
            println!("A number is required!");
            my_status = NumberParseResult::err;
            999
        },
    };
    return (my_num, my_status);
}

fn add_2_ints(a: i32, b: i32) -> i32 {
    return a+b;
}

enum NumberParseResult {
    ok,
    err,
    aborted
}