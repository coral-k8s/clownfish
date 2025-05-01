use gpui::Application;
mod components;

use components::Assets;

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        cx.activate(true);
    })
}
