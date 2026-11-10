use oop1::{Post, CanGetName};

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today.");

    println!("type_name:{}", post.type_name());
    

    let post = post.request_review();
    println!("type_name:{}", post.type_name());

    let post = post.approve();
    println!("type_name:{}", post.type_name());

    println!("Post:{}", post.content());


}

