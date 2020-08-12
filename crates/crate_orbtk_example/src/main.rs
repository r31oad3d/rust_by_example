use orbtk::prelude::*;

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - helloworld")
                .position((100.0, 100.0))
                .size(800.0, 600.0)
                .child(TextBlock::create().text("Hello World!").build(ctx))
                .build(ctx)
        })
        .run();
}
