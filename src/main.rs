use sfml::graphics::{
    Color, RenderTarget, RenderWindow, Font, Text, Transformable, Texture, Sprite
};
use sfml::system::{Vector2i, Vector2f};
use sfml::window::{ContextSettings, Event, Key, Style};

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

    // Load Resources
    let font = Font::from_file("lc_font.ttf").unwrap();
    let mut msg_main = Text::default();
    msg_main.set_font(&font);
    msg_main.set_character_size(80);
    msg_main.set_position(Vector2f::new(20., 20.));
    msg_main.set_fill_color(Color::GREEN);
    msg_main.set_string("Hello world!");

    let mut msg_sub_pos = 110.;
    let mut msg_sub_delta = 0.5;
    let mut msg_sub = Text::default();
    msg_sub.set_font(&font);
    msg_sub.set_character_size(30);
    msg_sub.set_position(Vector2f::new(20., msg_sub_pos));
    msg_sub.set_fill_color(Color::YELLOW);
    msg_sub.set_string("All your base are belong to us");

    let ship_texture = Texture::from_file("spaceship.png").unwrap();
    let mut ship = Sprite::new();
    ship.set_texture(&ship_texture, true);
    ship.set_position(Vector2f::new(305.,230.));
    ship.set_origin(Vector2f::new(35.,35.));
    let mut ship_rotation = 0.;

    // Main Loop
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyReleased { code, ctrl, shift, alt, .. } => {
                    println!("code={code:?} ctrl={ctrl:?} shift={shift:?} alt={alt:?}");
                    match code {
                        Key::ESCAPE => {
                            window.close();
                        },
                        Key::Q => {
                            if ctrl {
                                window.close();
                            }
                        },
                        _ => {}
                    }
                },
                _ => {} // ignore other events
            }
        }
        msg_sub_pos += msg_sub_delta;
        if msg_sub_pos > 400. || msg_sub_pos < 110. {
            msg_sub_delta = -msg_sub_delta;
        }
        ship_rotation += 1.;
        if ship_rotation >= 360. {
            ship_rotation = 0.;
        }
        ship.set_rotation(ship_rotation);
        msg_sub.set_position(Vector2f::new(20., msg_sub_pos));
        window.clear(Color::BLACK);
        window.draw(&ship);
        window.draw(&msg_main);
        window.draw(&msg_sub);
        window.display();
    }
}
