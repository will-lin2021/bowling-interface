use chrono::NaiveDate;

pub fn game_loop(date: NaiveDate, game: u8) {
    println!(
        "Date: {}, Game Number: {}\n",
        date.format("%m/%d/%y"),
        game_num,
    );

    let game_info = Info::build_with(date, game_num);
    let mut game: Game::build_with(game_info);
}

pub fn modify_loop() {

}
