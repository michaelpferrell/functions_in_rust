use std::io;

type UserIntType = i32;

fn main() {
    println!("This program calls different functions.");
    try_adding();
}

fn try_adding() {
    println!();
    println!("* Adding *");
    
    let a = try_getting_int_from_user("Please enter an integer for A.");
    println!("Entered for A: {}", a);
    
    let b = try_getting_int_from_user("Please enter an integer for B.");
    println!("Entered for B: {}", b);
    
    let my_sum = add_2_ints(a, b);
    println!("{} + {} = {}", a, b, my_sum);
    
}

fn try_getting_int_from_user(prompt: &str) -> UserIntType {
    loop {
        let (my_int, my_status) = try_getting_int_from_user_once(prompt);
        match my_status {
            UserInputStatus::Err => { continue; },
            UserInputStatus::Ok => { println!(); return my_int; },
        }
    }
}

fn try_getting_int_from_user_once(prompt: &str) -> (UserIntType, UserInputStatus) {
    let mut my_input = String::new();
    let my_status = get_line_from_user(prompt, &mut my_input);
    return match my_status {
        UserInputStatus::Err => (0, my_status),
        UserInputStatus::Ok => try_parsing_for_int(&mut my_input),
    };
}

fn get_line_from_user(prompt: &str, my_input: &mut String) -> UserInputStatus {
    println!("{}", prompt);
    io::stdin()
        .read_line(my_input)
        .expect("Failed to read line");
    return UserInputStatus::Ok;
}

fn try_parsing_for_int(my_input: &mut String) -> (UserIntType, UserInputStatus) {
    match my_input.trim().parse() {
        Ok(num) => {
            return (num, UserInputStatus::Ok)
        },
        Err(_) => {
            println!("An integer is required!");
            return (999, UserInputStatus::Err)
        },
    }
}

fn add_2_ints(a: UserIntType, b: UserIntType) -> UserIntType {
    return a+b;
}

enum UserInputStatus {
    Ok,
    Err
}
