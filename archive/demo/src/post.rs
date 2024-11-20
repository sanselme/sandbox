// SPDX-License-Identifier: GPL-3.0

pub struct Post {
    content: String,
}

// note: Post type methods
impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
}

// note: Post mutating methods
impl Post {
    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

// note: DraftPost consuming methods
impl DraftPost {
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

// note: DraftPost mutating methods
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

pub struct PendingReviewPost {
    content: String,
}

// note: PendingReviewPost consuming methods
impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
