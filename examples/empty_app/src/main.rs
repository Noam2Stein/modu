use modu::app::App;

fn main() {
    EmptyApp::run();
}

struct EmptyApp;

impl App for EmptyApp {
    fn init(_ctx: &modu::app::AppContext) -> Self {
        Self
    }
}
