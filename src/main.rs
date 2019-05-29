mod game;

fn main () {
    let x = game::create_word();
    let mut actual_Word: Vec<char> = x.chars().collect(); //Breaking the string into array
    let mut array_Guess = vec!['_'; actual_Word.len()-2];

    game::game_menu(&mut actual_Word, &mut array_Guess);
}