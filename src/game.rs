use std::io::{self,Write};

pub fn create_word() -> String {
    print!("Please enter a word: ");
    let mut input = String::new();    
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)
        .expect("Couldn't read line");
    input
}

pub fn game_menu(actual_Word: &mut Vec<char>, array_Guess: &mut Vec<char>)  {
    println!("You have 5 live to guess this word");
    let mut victory = false;
    let mut x = 5;
    while x != 0 {
        if play_game(actual_Word, array_Guess) != 1 {
            x = x - 1;
        }
        if checker(actual_Word, array_Guess) == 0 {
            victory = true;
            break;
        }
        println!("Remaining lives: {}", x);
    }
    if victory == true {
        println!("Congrat! You've won!");
    }
    else {
        println!("You've lost!");
    }
}

//This function will read in a word from the user
fn play_game(actual_Word: &mut Vec<char>, array_Guess: &mut Vec<char>) -> u8 {
    println!("Please guess a character:", );
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)
        .expect("Couldn't read line");
    let letter: Vec<char> = input.chars().collect(); //This is for the case of user being an asshole and type a string
    let key = letter[0];

    return compare(actual_Word, array_Guess, key);
}

//This function compares the guessed letter to see if it exists from the hidden word and 
//then assign that letter to the correct index
fn compare(actual_Word: &mut Vec<char>, array_Guess: &mut Vec<char>, key: char) -> u8 {
    let mut status = 0;
    for x in 0..actual_Word.len() - 3 {
        if key == actual_Word[x] {
            array_Guess[x] = key;
            status = 1;
        }
    }
    status
}
//This function compare the guess array with the actual word to see if all the correct letter is in place
fn checker(actual_Word: &mut Vec<char>, array_Guess: &mut Vec<char>) -> u8 {
    let mut flag = 0;
    for x in 0..actual_Word.len()-3 {
        if actual_Word[x] != array_Guess[x] {
            flag = 1;
        }
    }
    flag
}