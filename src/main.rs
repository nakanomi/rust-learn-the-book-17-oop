use oop1::Post;

fn main() {
    let msg = String::from("hello");
    println!("{}", msg);
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today.");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());
    assert_eq!("PendingReview", post.debug_state());

    post.reject();
    post.add_text("I ate a salad for lunch today.");


    post.approve();
    assert_eq!("I ate a salad for lunch today.", post.content());
    assert_eq!("Published", post.debug_state());

}

