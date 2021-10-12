///Author: Damien Frissant
use std::io::stdin;
fn main() {
    start_game();
}

fn start_game() {
    let size = 3;
    let mut vec: Vec<Vec<String>> = vec![vec![0.to_string(); size]; size];
    let mut value: i32 = 0;

    for i in 0..size {
        print!("+");

        let mut x = 0;
        while x < size {
            print!("---+");
            x += 1;
        }

        println!();
        print!("|");

        for j in 0..size {
            vec[i][j] = value.to_string();
            value += 1;
            if (value) > 10 {
                print!("{} |", vec[i][j]);
            } else {
                print!(" {} |", vec[i][j]);
            }
        }
        println!();
    }
    let mut x = 0;
    print!("+");
    while x < size {
        print!("---+");
        x += 1;
    }
    println!();

    /*for i in 0..size{

    }*/

    let mut i = 0;

    let to_flatted: Vec<Vec<String>> = vec.clone();

    //Convert 2D matrix in 1D matrix
    let flattened = to_flatted.into_iter().flatten().collect::<Vec<String>>();
    let mut player = true;

    while i < (size * size) {
        let line = &*get_keypad();
        let convert: u8 = line.parse().unwrap();

        if convert as usize > (size * size - 1) {
            println!("Error, the matrix is too big, please choose another number");
        } else {
            //Get if the entering number is in "flattened"
            let present = flattened.iter().any(|x| x.to_string() == line.to_string());
            //Get the row by divided the position of the flat matrix by the nb of row (size)
            let row_index = (flattened.iter().position(|r| r == line).unwrap()) / size;
            println!("{}", row_index);
            let line_index: usize = 0;
            if present {
                //Get the line index of the number
                let line_index = vec[row_index]
                    .iter()
                    .position(|r| r.to_string() == line.to_string())
                    .unwrap();
                println!("Value 2 present at: [{}][{}]", row_index, line_index);
                println!("Valeur de vec: {}", vec[row_index][line_index]);
            } else {
                println!("This cell is already used, please choose another one");
            }
            if player {
                vec[row_index][line_index] = "X".to_string();
                player = false;
                println!("Valeur de vec: {}", vec[row_index][line_index]);
            } else {
                vec[row_index][line_index] = "O".to_string();
                player = true;
                println!("Valeur de vec: {}", vec[row_index][line_index]);
            }
            //println!("\x1B[2J\x1B[1;1H");
            print_new_grid(& mut vec, &size);
            
            /*if check_win(&vec) {
                println!("You win !");
                return;
            }*/

            i += 1;
            println!("Valeur en 2;2 : {}", vec[1][1].to_string());
            print!("Valeur en 2;2 : {}", vec[2][2].to_string());
        }
    }
}

fn get_keypad() -> String {
    let mut key_entry = String::new();

    stdin()
        .read_line(&mut key_entry)
        .expect("Couldn’t read line from stdin");
    key_entry.to_lowercase();
    key_entry.replace("\n", "").replace("\r", "")
}

fn print_new_grid(vec: & mut Vec<Vec<String>>, size: &usize) {
    for i in 0..*size {
        print!("+");

        let mut x = 0;
        while x < *size {
            print!("---+");
            x += 1;
        }

        println!();
        print!("|");

        for j in 0..*size {
            print!(" {} |", vec[i][j]);
        }
        println!();
    }
    let mut x = 0;
    print!("+");
    while x < *size {
        print!("---+");
        x += 1;
    }
    println!();
}

fn check_win(numbers: &Vec<String>) -> bool {
    //Line win possibilities
    (numbers[0] == numbers[1] && numbers[1] == numbers[2] &&  numbers[2] == "X") ||
    (numbers[0] == numbers[1] && numbers[1] == numbers[2] &&  numbers[2] == "O") ||

    (numbers[3] == numbers[4] && numbers[4] == numbers[5] &&  numbers[5] == "X") ||
    (numbers[3] == numbers[4] && numbers[4] == numbers[5] &&  numbers[5] == "O") ||

    (numbers[6] == numbers[7] && numbers[7] == numbers[8] &&  numbers[8] == "X") ||
    (numbers[6] == numbers[7] && numbers[7] == numbers[8] &&  numbers[8] == "O") ||

    //Row win possibilities
    (numbers[0] == numbers[3] && numbers[3] == numbers[6] &&  numbers[6] == "X") ||
    (numbers[0] == numbers[3] && numbers[3] == numbers[6] &&  numbers[6] == "O") ||

    (numbers[4] == numbers[1] && numbers[7] == numbers[1] &&  numbers[7] == "X") ||
    (numbers[4] == numbers[1] && numbers[7] == numbers[1] &&  numbers[7] == "O") ||

    (numbers[2] == numbers[5] && numbers[5] == numbers[8] &&  numbers[8] == "X") ||
    (numbers[2] == numbers[5] && numbers[5] == numbers[8] &&  numbers[8] == "O") ||

    //Diagoanal win possibilities
    (numbers[0] == numbers[4] && numbers[8] == numbers[4] &&  numbers[0] == "X") ||
    (numbers[0] == numbers[4] && numbers[8] == numbers[4] &&  numbers[0] == "O") ||

    (numbers[2] == numbers[4] && numbers[4] == numbers[6] &&  numbers[6] == "X") ||
    (numbers[2] == numbers[4] && numbers[4] == numbers[6] &&  numbers[6] == "O")
}
