// TODO: Implement pong

use modu::app::App;

fn main() {
    Game::run();
}

struct Game {}

impl App for Game {
    fn config(config: &mut modu::app::AppConfig) {
        config.window_attributes.title = "Pong".to_string();
    }

    fn init(_ctx: &modu::app::AppContext) -> Self {
        Self {}
    }
}
