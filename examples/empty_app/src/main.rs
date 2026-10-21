use modu::app::App;

fn main() {
    EmptyApp::run();
}

struct EmptyApp;

impl App for EmptyApp {
    fn initialize(_ctx: &modu::app::AppContext) -> Self {
        Self
    }
}
