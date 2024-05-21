pub fn draw_line(_x: i32, _y: i32) {
    // draw line without color
}

#[cfg(feature = "color")] // this line means that the color module is enabled, if the color feature is enabled
pub mod color {
    pub use rgb::RGB;

    pub fn draw_line(_x: i32, _y: i32, color: &RGB<u16>) {
        println!("{color}");

        //draw line with color
    }
}

#[cfg(feature = "shapes")] // this line means that the shapes module is enabled, if the shapes feature is enabled
pub mod shapes {
    use rgb::RGB;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Rectangle {
        pub color: RGB<u16>,
        pub width: u32,
        pub height: u32,
    }
}
