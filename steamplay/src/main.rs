mod util;

fn main() {
    println!("steamplay - 0.0.1");
    if let Some(game_path) = util::fs::select_game("noita.exe") {
        println!("game path : {:?}", game_path);
        util::fs::start_game(game_path);
    }
}
