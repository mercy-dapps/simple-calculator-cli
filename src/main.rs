use std::io;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// calculation options
enum Calculation {
    Basic,
    Shape
}

// basic calculation options
#[derive(Debug, EnumIter)]
enum Basic {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
    Square,
    Squareroot,
    Percentage
}

// shape calculation options
#[derive(Debug, EnumIter)]
enum Shape {
    Circle,
    Square,
    Rectangle,
    Triangle
}

fn main() {
    println!("Welcome, you can ask me simple calculation questions");

    println!("To begin, select the calculation you want");

    println!("Enter 1 for Basic calculations -> +, *, -, /, %, square, squareroot, percentage");
    println!("Enter 2 for Shape calculations -> Area of circle, rectangle, triangle, square");

    let mut select_cal_option = String::new();

    io::stdin()
    .read_line(&mut select_cal_option)
    .expect("Failed to read input");

    let selected_cal = return_selected_cal(&select_cal_option);

    let format_selection = match selected_cal {
        Calculation::Basic => String::from("Basic"),
        Calculation::Shape => String::from("Shape"),
    };

    println!("You selected {} calculations, choose option below: ", format_selection);

    if format_selection == "Basic" {
       for (i, basic) in Basic::iter().enumerate() {
        println!("select {} for {:?}", {i + 1}, basic);
        } 
    }
    else {
        for (i, shape) in Shape::iter().enumerate() {
        println!("select {} to find the area of {:?}", {i + 1}, shape);
        } 
    }

    let mut select_cal_option = String::new();
    
    io::stdin()
    .read_line(&mut select_cal_option)
    .expect("Failed to read input");

    if !select_cal_option.trim().is_empty() {

        if format_selection == "Basic" {
            let selected_basic = return_selected_basic_cal(&select_cal_option);
        
        match selected_basic {
            Basic::Addition => {
                println!("You selected Addition calculations, Enter two values separated by comma");
                let mut addition_values = String::new();

                io::stdin()
                .read_line(&mut addition_values).unwrap();

                let split_input: Vec<i32> = addition_values
                .trim()
                .split(',')
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect();

                let result = addition(split_input[0], split_input[1]);
                println!("The addition result is {}", result);
            },
            Basic::Subtraction => 
            {
                println!("You selected Subtraction calculations, Enter two values separated by comma");
                let mut subtraction_values = String::new();

                io::stdin()
                .read_line(&mut subtraction_values).unwrap();

                let split_input: Vec<i32> = subtraction_values
                .trim()
                .split(',')
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect();

                let result = subtraction(split_input[0], split_input[1]);
                println!("The subtraction result is {}", result);
            },
            Basic::Division => {
            println!("You selected Division calculations, Enter two values separated by comma");
            let mut division_values = String::new();

                io::stdin()
                .read_line(&mut division_values).unwrap();

                let split_input: Vec<f64> = division_values
                .trim()
                .split(',')
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect();

                let result = division(split_input[0], split_input[1]);
                println!("The division result is {}", result);
        },
            Basic::Multiplication =>{
                println!("You selected Multiplication calculations, Enter two values separated by comma");
                let mut multiplication_values = String::new();

                io::stdin()
                .read_line(&mut multiplication_values).unwrap();

                let split_input: Vec<i32> = multiplication_values
                .trim()
                .split(',')
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect();

                let result = multiplication(split_input[0], split_input[1]);
                println!("The multiplication result is {}", result);
            },
            Basic::Square => {
                println!("You selected Square calculations, Enter a value to square");
                let mut square_values = String::new();

                io::stdin()
                .read_line(&mut square_values).unwrap();

                let result = square(square_values.trim().parse::<i32>().unwrap());
                println!("The square result is {}", result);
            },
            Basic::Squareroot => {
                println!("You selceted Squareroot calculations, Enter a value to find the squareroot");
                let mut squareroot_values = String::new();

                io::stdin()
                .read_line(&mut squareroot_values).unwrap();

                let result = squareroot(squareroot_values.trim().parse::<i32>().unwrap());

                match result {
                    Some(res) =>  println!("The squareroot result is {}", res),
                    None =>  println!("Number must be greater than 0")
                }
            },
            Basic::Modulus => {
                println!("You selected Modulus calculations, Enter two values separated by comma");
                let mut modulus_values = String::new();

                io::stdin()
                .read_line(&mut modulus_values).unwrap();

                let split_input: Vec<i32> = modulus_values
                .trim()
                .split(',')
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect();

                let result = modulus(split_input[0], split_input[1]);
                println!("The modulus result is {}", result);
            },
            Basic::Percentage => {
                println!("You selected Percentage calculations, Enter two values separated by comma e.g 20, 100 reads 20% of 100");
                let mut percentage_values = String::new();

                io::stdin()
                .read_line(&mut percentage_values).unwrap();

                let split_input: Vec<i32> = percentage_values
                .trim()
                .split(',')
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect();

                let result = percentage(split_input[0], split_input[1]);
                println!("The percentage result is {}", result);
            },
        }; 
        }

        else {
             let selected_shape = return_selected_shape_cal(&select_cal_option);

            match selected_shape {
            Shape::Circle => {
                println!("You selected Circle, Enter radius");
                let mut circle_values = String::new();

                io::stdin()
                .read_line(&mut circle_values).unwrap();
               
                let result = area_of_circle(circle_values.trim().parse::<f64>().unwrap());
                println!("The circle result is {}", result);
            },
            Shape::Square => 
            {
                println!("You selected Square, Enter the side of the square");
                let mut square_values = String::new();

                io::stdin()
                .read_line(&mut square_values).unwrap();

                let result = area_of_square(square_values.trim().parse::<f64>().unwrap());
                println!("The square result is {}", result);
            },
            Shape::Rectangle => {
            println!("You selected Rectangle, Enter the length and breadth values separated by comma");
            let mut rectangle_values = String::new();

                io::stdin()
                .read_line(&mut rectangle_values).unwrap();

                let split_input: Vec<f64> = rectangle_values
                .trim()
                .split(',')
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect();

                let result = area_of_rectangle(split_input[0], split_input[1]);
                println!("The rectangle result is {}", result);
        },
            Shape::Triangle =>{
                println!("You selected Triangle, Enter the base and height values separated by comma");
                let mut triangle_values = String::new();

                io::stdin()
                .read_line(&mut triangle_values).unwrap();

                let split_input: Vec<f64> = triangle_values
                .trim()
                .split(',')
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect();

                let result = area_of_triangle(split_input[0], split_input[1]);
                println!("The triangle result is {}", result);
            }
        };  
        }
    }
}

fn return_selected_cal(option: &str) -> Calculation {
    match option.trim() {
        "1" => Calculation::Basic,
        "2" => Calculation::Shape,
        _ => {
            println!("Invalid input. Exiting..");
            std::process::exit(1);
        }
    }
}

fn return_selected_basic_cal(option: &str) -> Basic {
    match option.trim() {
        "1" => Basic::Addition,
        "2" => Basic::Subtraction,
        "3" => Basic::Multiplication,
        "4" => Basic::Division,
        "5" => Basic::Modulus,
        "6" => Basic::Square,
        "7" => Basic::Squareroot,
        "8" => Basic::Percentage,
        _ => {
            println!("Invalid input. Exiting..");
            std::process::exit(1);
        }
    }
}

fn return_selected_shape_cal(option: &str) -> Shape {
    match option.trim() {
        "1" => Shape::Circle,
        "2" => Shape::Square,
        "3" => Shape::Rectangle,
        "4" => Shape::Triangle,
        _ => {
            println!("Invalid input. Exiting..");
            std::process::exit(1);
        }
    }
}

// ALL BASIC CALCULATIONS

// addition
fn addition(a: i32, b: i32) ->i32 {
    return a + b;
}

// subtraction
fn subtraction(a: i32, b: i32) ->i32 {
    return a - b;
}

// multiplication
fn multiplication(a: i32, b: i32) ->i32 {
    return a * b;
}

// division
fn division(a: f64, b: f64) ->f64 {
    return a / b as f64;
}

// modulus
fn modulus(a: i32, b: i32) ->i32 {
    return a % b;
}

// square
fn square(a: i32) ->i32 {
    return a.pow(2);
}

// squareroot
fn squareroot(a: i32) ->Option<f64> {
    if a.is_positive() && a  > 0 {
        return Some((a as f64).sqrt())
    }
    else {
       return None
    }
}

// percentage
fn percentage(a: i32, b: i32) ->i32 {
    return (a * b)/100;
}

// ALL SHAPE CALCULATIONS

// circle
fn area_of_circle(r: f64) -> f64 {
    return std::f64::consts::PI * r * r 
}

// circle
fn area_of_square(side: f64) -> f64 {
    return side * side
}

// circle
fn area_of_rectangle(l: f64, b: f64) -> f64 {
    return l * b 
}

// circle
fn area_of_triangle(b: f64, h: f64) -> f64 {
    return 0.5 * b * h
}