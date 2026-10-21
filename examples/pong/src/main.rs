// TODO: Implement pong

use modu::app::App;

fn main() {
    Game::run();
}

struct Game {}

impl App for Game {
    fn configure(config: &mut modu::app::AppConfig) {
        config.window_attributes.title = "Pong".to_string();
    }

    fn initialize(_ctx: &modu::app::AppContext) -> Self {
        Self {}
    }
}
