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
    // デバッグ用に自分の型名を返すメソッド（デフォルト実装）
    fn type_name(&self) -> &'static str;
}

fn short_name<T>() -> &'static str {
    let full_name = type_name::<T>();
    full_name.split("::").last().unwrap()
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

    fn type_name(&self) -> &'static str {
        short_name::<Self>()
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
    fn type_name(&self) -> &'static str {
        short_name::<Self>()
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
    fn type_name(&self) -> &'static str {
        short_name::<Self>()
    }
}