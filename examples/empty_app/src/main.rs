use modu::app::App;

fn main() {
    EmptyApp::run();
}

struct EmptyApp;

impl App for EmptyApp {
    fn new(_ctx: &modu::app::AppContext) -> Self {
        Self
    }
}
