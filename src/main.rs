use std::io;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;



enum Calculation {
    Basic,
    Shape
}

#[derive(Debug, EnumIter)]
enum Basic {
    Addition,
    Substraction,
    Multiplication,
    Division,
    Modulus,
    Square,
    Squareroot,
    Percentage
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

  for (i, basic) in Basic::iter().enumerate() {
        println!("{} for {:?}", {i + 1}, basic);
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
