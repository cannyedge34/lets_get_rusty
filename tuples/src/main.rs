fn main() {
    // tuples
    let rgb_color = (255, 106, 0);
    let cmyk_color = (0, 58, 100, 0);

    // tuple structs
    struct RGB(i32, i32, i32);
    struct CMYK(i32, i32, i32, i32);

    let rgb_color = RGB(255, 106, 0);
    let cmyk_color = CMYK(0, 58, 100, 0);

    // unit-like structs
    struct MyStruct;
}
