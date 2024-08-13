struct Tweet<'a> {
    /*
        we would get an error missing lifetime...
        we need to add generic lifetime annotations storing references in a struct
    */
    content: &'a str
}

impl<'a> Tweet<'a> {
    fn replace_content(&mut self, content: &'a str) -> &str {
        let old_content = self.content;
        self.content = content;
        old_content
    }
}

fn main() {
    let mut tweet = Tweet {
        content: "example"
    };

    let old_content = tweet.replace_content("replace_example");
    println!("{old_content}");
    println!("{}", tweet.content);
}

/*
    we have input lifetimes and output lifetimes
    1. input lifetimes for input params
    2. output lifetime for output param

    3 lifetimes elision rules:

    1. Each parameter that is a reference gets its own lifetime parameter.
    2. If there is exactly one input lifetime parameter, that lifetime
    is assigned to all output lifetime parameters.
    3 if there are multiple input lifetime parameters, but one of them is
    &self or &mut self, the lifetime of self is assigned to all output
    lifetime parameters

    case 1 one input parameter:
    fn take_and_return_content<'a>(content: &'a str) -> &'a str {
        content
    }

    case 2 multiple input parameters
    fn take_and_return_content<'a>(content: &'a str, content2: &'a str) -> &'a str {
        content
    }
*/
