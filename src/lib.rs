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
        self.content.push_str(text);
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
            Some(s) => s.type_name(),
            None => "None",
        }
    }
}

trait  State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>)-> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) ->&'a str {
        ""
    }
    fn reject(self: Box<Self>, post: &mut Post)-> Box<dyn State>;
}

pub trait StateName {
    fn type_name(&self) -> &'static str;
}

impl <T: State + ?Sized + 'static> StateName for T {
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
        Box::new(Published{})
    }
    fn reject(self: Box<Self>, post: &mut Post)-> Box<dyn State> {
       post.content = String::new();
       Box::new(Draft{})
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
    fn reject(self: Box<Self>, post: &mut Post)-> Box<dyn State> {
        self
    }
}