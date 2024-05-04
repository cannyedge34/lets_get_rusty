struct Product {
    name: String,
    price: f32,
    in_stock: bool
}

impl Product {
    // this is a common pattern in rust using a constructor or initialize
    fn new(name: String, price: f32) -> Product {
        Product {
            name: name,
            price: price,
            in_stock: true
        }
    }

    /*
        this is an associated function
        associated functions do not take self as a parameter
    */
    fn get_default_sales_tax() -> f32 {
        0.1
    }

    // we add this method in product instances sending
    // &self as argument
    // this &self is used when we don't need to modify anything
    fn calculate_sales_tax(&self) -> f32 {
        // self.price * 0.1
        // with an associated function:
        self.price * Product::get_default_sales_tax()
    }

    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    fn buy(self) -> i32 {
        let name = self.name;
        println!("{name} was bought");
        123
    }
}

fn main() {
    // instance of product
    // let mut book = Product {
    //     name: String::from("Book"),
    //     price: 30.20,
    //     in_stock: true
    // };

    /*
        we can use the constructor in line 9 to create a product instance
        instead of calling Product in line 47
        this way is cleaner than calling Product directly
    */

    let mut book = Product::new(
        String::from("Book"),
        30.0
    );

    // let price = book.price;
    // book.in_stock =  false;

    // this is without impl block
    // let sales_tax = calculate_sales_tax(&book);

    let sales_tax = book.calculate_sales_tax();
    println!("{}", sales_tax);

    book.set_price(1.0);
    book.buy();

    /*
        we have an error setting the price again in line 53, since
        the "buy" method takes the own form of "self" and it means
        the instance of book will be moved into the buy method
        and dropped it. After that, we call the buy method, the book instance
        is not longer valid
    */
    // book.set_price(2.0)

}

// We can move this function into the impl block
// fn calculate_sales_tax(product: &Product) -> f32 {
//     product.price * 0.1
// }
