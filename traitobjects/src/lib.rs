
pub struct Post{
    contents: String,
    state: Option<Box<dyn State>>
}

impl Post {
    pub fn new() -> Self {
        Post{contents: String::new(), state: Some(Box::new(Draft{}))}
    }
    
    pub fn contents(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    
    pub fn add_text(&mut self, text: &str) {
        self.contents.push_str(text)
    }
    
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
    
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, p: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }
    
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, p: &'a Post) -> &'a str {
        &p.contents
    }
    
}
