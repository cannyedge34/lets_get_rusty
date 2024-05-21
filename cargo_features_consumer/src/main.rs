use cargo_features::color;
use draw::color;

fn main() {
    cargo_features::draw_line(32, 32);

    let color: color::RGB {
        r: 247,
        g: 76,
        b: 0
    }

    color::draw_line(32, 32, &color);

    let square = draw::shapes::Rectangle {
        color,
        with: 32,
        height: 32,
    };

    println!("{square:?}");
}
