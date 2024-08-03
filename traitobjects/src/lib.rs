
pub struct Post{
    contents: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost{content: String::new()}
    }
    
    pub fn contents(&self) -> &str {
        &self.contents
    }
}

pub struct DraftPost {
    content: String
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }

    pub fn request_review(self) -> PendingReview {
        PendingReview { content: self.content }
    }
}

pub struct PendingReview {
    content: String
}
impl PendingReview {
    pub fn approve(self) -> Post {
        Post {contents: self.content}
    }

}
