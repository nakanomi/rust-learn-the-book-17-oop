use oop1::{Post, StateName};

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today.");
    assert_eq!("DraftPost", post.type_name());
    

    let post = post.request_review();
    assert_eq!("PendingReviewPost", post.type_name());

    let post = post.approve();
    assert_eq!("PublishedPost", post.type_name());

    println!("Post:{}", post.content());


}

