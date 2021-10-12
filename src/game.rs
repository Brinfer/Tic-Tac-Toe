///Author: Damien Frissant
use std::io::stdin;
fn main() {
    start_game();
}

fn start_game() {
    let size = 3 * 3;
    let mut numbers: Vec<String> = Vec::new();

    for i in 1..(size + 1) {
        numbers.push(i.to_string());
        println!("{:?}", numbers);
    }

    println!("\x1B[2J\x1B[1;1H");
    //TODO auto generate the grid according to the size
    println!(
        " Grille d'initialisation:\n
+---+---+---+
| {} | {} | {} |
+---+---+---+
| {} | {} | {} |
+---+---+---+
| {} | {} | {} |
+---+---+---+
\nEntrer le nombre où vous souhaitez placer votre symbole\n",
        numbers[0],
        numbers[1],
        numbers[2],
        numbers[3],
        numbers[4],
        numbers[5],
        numbers[6],
        numbers[7],
        numbers[8]
    );

    let mut i = 0;
    let mut player: bool = true;
    while i < size {
        let line = &*get_keypad();
        //Look into numbers and return a bool if the variable 'line' is in
        let present = numbers.iter().any(|x| x == line);
        //println!("Est present ? {}", present);

        if present {
            let index = numbers.iter().position(|r| r == line).unwrap();
            //println!("Le {:?} se trouve en {:?} position", line, index);

            if player {
                numbers[index] = "X".to_string();
                player = false;
            } else {
                numbers[index] = "O".to_string();
                player = true;
            }
            println!("\x1B[2J\x1B[1;1H");
            print_new_grid(&numbers);
            if check_win(&numbers) {
                println!("You win !");
                return;
            }
            i += 1;
        } else {
            println!("This cell is already used, please choose another one");
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

fn print_new_grid(numbers: &Vec<String>) {
    println!(
        "
+---+---+---+
| {} | {} | {} |
+---+---+---+
| {} | {} | {} |
+---+---+---+
| {} | {} | {} |
+---+---+---+
",
        numbers[0],
        numbers[1],
        numbers[2],
        numbers[3],
        numbers[4],
        numbers[5],
        numbers[6],
        numbers[7],
        numbers[8]
    );
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
