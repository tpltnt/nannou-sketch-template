use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(GREEN);
    draw.to_frame(app, &frame).unwrap();
}
