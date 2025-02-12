use nerf::prelude::*;

fn main() {
    let app = App::new(
        Align::new(
            Alignment::CENTER,
            Text::new(
                "HELLO,\nRust! 🦀".to_string(),
                TextStyle::default()
                    .colored(Color::WHITE),
            ),
        )
    );

    app.run()
}
