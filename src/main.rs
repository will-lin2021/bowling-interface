use dotenvy::dotenv;

use bowling_interface::app::tui;

fn main() {
    dotenv().expect(".env not found");

    let mut app = tui::App::default();
    let _res = tui::run(&mut app);
}
