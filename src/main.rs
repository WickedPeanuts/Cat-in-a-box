mod entity2d;
mod scene_texture_manager;
mod game;

use crate::game::Game;

fn main() {
    let mut game = Game::new();
    game.start_operation();
}

/*fn bible() {
        // types
    /* */
    let a: bool = true;
    let c: char = 'c';
    let f32: f32 = 0.01_f32;
    let i32: i32 = 100_i32;
    let tuple: (i32, &str) = (10, "123");
    let a: &'static i32 = &100;

    let mut tuple_elem: String = tuple.1.to_string() + "321";
    let str_2: String = tuple_elem;
    let arr: [i32; 3] = [1,2,3];

    let mut list: Vec<i32> = vec!(1, 2, 3, 4);
    list.push(10);
}

fn hello_carai() {
    let stdout = stdout();
    let message = String::from("Hello, carai");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}*/