use nerf::prelude::*;

fn main() {
    let app = App::new(
        Align::new(
            Alignment::CENTER,
            Button::new(
                SizedBox::new(
                    200, 80,
                    Background::new(Color::rgb(0, 0, 255), Empty::expand())
                )
            )
        )
    );

    app.run()
}
