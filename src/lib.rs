use std::any::type_name;
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if let Some(s) = &self.state {
            if (*s).is_rewritable() {
                self.content.push_str(text);
            } else {
                println!("This post cannot be edited from its current state.")
            }

        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject(self))
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn debug_state(&self) -> &str {
        match &self.state {
            Some(s) => (*s).type_name(),
            None => "None",
        }
    }
}

/// 名前を取得するトレイト
pub trait StateName {
    fn type_name(&self) -> &'static str;
}
/// ステートトレイト
trait  State : StateName {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>)-> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) ->&'a str {
        ""
    }
    fn is_rewritable(&self) -> bool {
        true
    }
    fn reject(self: Box<Self>, post: &mut Post)-> Box<dyn State>;
}

/// Stateかつ、ある程度の条件を備えた構造体に対してのStateNameの実装
impl <T: State + Sized + 'static> StateName for T {
    fn type_name(&self) -> &'static str {
        let full_name = std::any::type_name_of_val(self);
        full_name.split("::").last().unwrap()
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>)-> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>, post: &mut Post)-> Box<dyn State> {
        self
    }

}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>)-> Box<dyn State> {
        Box::new(OneReviewed{})
    }
    fn reject(self: Box<Self>, post: &mut Post)-> Box<dyn State> {
       post.content = String::new();
       Box::new(Draft{})
    }
}

struct OneReviewed {}

impl State for OneReviewed {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>)-> Box<dyn State> {
        Box::new(Published{})
    }
    fn reject(self: Box<Self>, post: &mut Post)-> Box<dyn State> {
        self
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>)-> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) ->&'a str {
        &post.content
    }
    fn is_rewritable(&self) -> bool {
        false
    }
    fn reject(self: Box<Self>, post: &mut Post)-> Box<dyn State> {
        self
    }
}