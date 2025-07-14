use std::io;
use rand::Rng;

fn print_board(board: &[[i32; 10]; 10]) {
    for i in 0..9 {
        println!("{:?}", board[i]);
    }
}

fn place_enemy_ship(board: &mut [[i32; 10]; 10]) {
    let mut rng = rand::thread_rng();
    let random_x = rng.gen_range(0..9);
    let random_y = rng.gen_range(0..9);
    if board[random_x][random_y] == 1 {
        // If the position is already occupied, try again
        place_enemy_ship(board);
        return;
    }
    board[random_x][random_y] = 1; // Set the random position to 1 to represent a ship
}

fn main() {
    let is_game_over: bool = false;

    // Create a 10x10 empty board
    let mut board: [[i32; 10]; 10] = [
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
    ];

    let mut guess_board: [[i32; 10]; 10] = [
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
    ];

    println!("Welcome to battleships!");

    // Place an enemy ship randomly on the board
    for i in 0..5 {
        place_enemy_ship(&mut board);
    }

    // Game loop
    while !is_game_over {
        //print_board(&board); // Print the initial board

        // Input the x coordinate of the guess
        let mut guess_x = String::new();
        println!("Enter the x coordinate of your guess (0-9):");
        io::stdin()
            .read_line(&mut guess_x)
            .expect("Failed to read line");
        let guess_x: i32 = guess_x
            .trim()
            .parse()
            .expect("Please type a whole number!");   

        // Input the y coordinate of the guess
        let mut guess_y = String::new();
        println!("Enter the y coordinate of your guess (0-9):");
        io::stdin()
            .read_line(&mut guess_y)
            .expect("Failed to read line");
        let guess_y: i32 = guess_y
            .trim()
            .parse()
            .expect("Please type a whole number!");

        // Check if the guessed position is a hit or miss
        if board[guess_y as usize][guess_x as usize] == 1 {
            println!("Hit!");
            guess_board[guess_y as usize][guess_x as usize] = 2; // Mark the hit position with 2
            print_board(&guess_board);
        } else {
            println!("Miss!");
            guess_board[guess_y as usize][guess_x as usize] = 1; // Mark the missed position with 'x'
            print_board(&guess_board);
        }
    }
}