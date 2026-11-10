use oop1::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today.");
    

    let post = post.request_review();

    let post = post.approve();

    println!("Post:{}", post.content());


}

