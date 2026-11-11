use std::any::type_name;
pub struct  Post {
    content : String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost { content:
             String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text : &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content : self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

pub struct PublishedPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> PublishedPost{
        PublishedPost { content: self.content }
    }
}

impl PublishedPost {
    pub fn content(&self) -> &str {
        &self.content
    }
    
}
pub trait StateName {
    fn type_name(&self) -> &'static str;
}

/// Stateかつ、ある程度の条件を備えた構造体に対してのStateNameの実装
impl <T: Sized + 'static> StateName for T {
    fn type_name(&self) -> &'static str {
        let full_name = std::any::type_name_of_val(self);
        full_name.split("::").last().unwrap()
    }
}

