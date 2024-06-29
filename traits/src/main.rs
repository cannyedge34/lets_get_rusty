/*
    In this code Paint is a superTrait of vehicle now with the syntax "trait Vehicle: Paint"

    trait Vehicle: Paint {
    fn park(&self);

    fn get_default_color() -> String {
        "blank".to_owned()
    }
}
*/
trait Park {
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: String) {
        print!("painting object: {}", color)
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16
}

struct Car {
    info: VehicleInfo
}

impl Park for Car {
    fn park(&self) {
        println!("parking a car");
    }
}

impl Paint for Car {}

impl Park for Truck {
    fn park(&self) {
        println!("parking a truck");
    }
}


struct Truck {
    info: VehicleInfo
}

impl Truck {
    fn unload(&self) {
        println!("unloading truck")
    }
}

impl Paint for Truck {}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("painting house: {}", color);
    }
}

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Dacia".to_owned(),
            model: "Sandero".to_owned(),
            year: 2015
        }
    };

    let house = House {};

    let object = create_paintable_object();

    paint_red(&car);
    paint_red(&house);
    paint_red(&object);

    paint_vehicle_red(&car);
    // paint_vehicle_red(&house); error because house object do not implement the Park trait
    // paint_vehicle_red(&object); error because house object do not implement the Park trait
}

// first way of using trait bound with <T: Paint>
fn paint_red<T: Paint>(object: &T) {
    object.paint("red".to_owned());
}

/*
    second way of using trait bound with impl <T: & impl Paint>
    syntax sugar of the previous paint_red syntax
    object must be a reference to something that implements the Paint trait
*/
fn paint_red_impl_syntax(object: &impl Paint) {
    object.paint("red".to_owned());
}


/*
    This way is easier to read if we have multiple trait bounds
    for example paint_vehicle_red. T must something that implements Paint and Park traits
*/
fn paint_vehicle_red<T>(object: &T) where T: Paint + Park {
    object.paint("red".to_owned());
}

fn create_paintable_object() -> impl Paint {
    House {}
}