use std::fmt::Debug;
use std::fmt::Display;

struct Data<T>{
    value: T,
}

impl<T> Data<T>
where 
    T: Debug + Display
{
    fn print_me(&self){
        println!("My value: {}", self.value);
    }
}
fn main() {
    let data = Data {value : '2'};
    let data2 = Data {value: 1.0};
    let data3 = Data {value: 3};
    data.print_me();
    data2.print_me();
    data3.print_me();

    let fb_post = FacebookPost {
        author: String::from("Peter"),
        content: String::from("Sadly I'm not funny")
    };

    let ig_post = InstagramPost {
        author: String::from("Hans"),
        description: String::from("Not a funny description of an instagram post!")
    };


    print_summary(&fb_post);
    print_summary(&ig_post);

    let mut b = Data{value: 0};

    b.print_me();
    let mut test = 4;
    b.contain(test);
    test = 5;
    println!("{}", test);
    b.print_me();

}

struct FacebookPost{
    author: String,
    content: String
}

struct InstagramPost{
    author: String,
    description: String
}


impl Summary for FacebookPost {
    fn summarize(&self) -> String {
        format!("{}:\n\t{}", self.author, self.content)
    }
}


impl Summary for InstagramPost {
    fn summarize(&self) -> String {
        format!("{}:\n\t{}", self.author, self.description)
    }
}

fn print_summary(post: &impl Summary){
    println!("{}", post.summarize());
}

trait Summary {
    fn summarize(&self) -> String;
}

impl<T> Container<T> for Data<T>{
    fn contain(&mut self, value: T) {
        self.value = value;
    }
}


trait Container<T> {
    fn contain(&mut self, value: T);
}