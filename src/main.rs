use sfml::graphics::{
    Color, RectangleShape, RenderTarget, RenderWindow, Font, Shape, Sprite, Text, Transformable, View,
};
use sfml::system::{Clock, Vector2i, Vector2f};
use sfml::window::{mouse, ContextSettings, Event, Key, Style};

fn main() {
    let mut window = RenderWindow::new(
        (640, 480),
        "Hello world",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);
    window.set_position(Vector2i::new(50, 50));

    let font = Font::from_file("lc_font.ttf").unwrap();
    let mut msg = Text::default();
    msg.set_font(&font);
    msg.set_character_size(40);
    msg.set_position(Vector2f::new(20., 20.));
    msg.set_fill_color(Color::GREEN);
    msg.set_string("Hello world!");
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::KeyReleased { code, .. } => {
                    if code == Key::ESCAPE {
                        window.close();
                    }
                },
                _ => {} // ignore other events
            }
        }
        window.draw(&msg);
        window.display();
    }
}
