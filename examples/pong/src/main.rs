// TODO: Implement pong

use modu::app::App;

fn main() {
    Game::run();
}

struct Game;

impl App for Game {
    fn new(_ctx: &modu::app::AppContext) -> Self {
        Self
    }
}
