trait UIComponent {
    fn render(&self) {
        println!("Rendering component...")
    }
}
struct Button {
    text: String
}
impl UIComponent for Button {}

struct Container {
    name: String,
    child: Box<Container>
}

fn main() {
    let button_a = Button { text: "button".to_owned() }; // stored on the stack

    /*
        There are a few use_cases for the box smart pointer

    */

    // 1. Avoid copying large amount of data when transferring ownership
    let button_b = Box::new(Button { text: "button b".to_owned() }); // stored on the heap

    let button_c = button_a;
    let button_d = button_b;

    let components: Vec<Box<dyn UIComponent>> = vec![
        Box::new(button_c),
        button_d
    ];

    /*
        2. When we have a type of unknown size
        and we want to use it in a context where the exact size is required

        for example a recursive type in Container struct of this file
        we get the error: ^^^^^^^^^^^^^^^^ recursive type has infinite size

        The solution is to put "child" field of Container struct behind
        some kind of pointer, for example: Box<Container>

        this works because the size of child is the size of Box
        which has a known size, it's just a simple pointer!
    */

}
