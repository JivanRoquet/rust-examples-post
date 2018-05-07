use std::fmt::{Display, Formatter, Result};

pub trait PostHistory {
    fn history(&self) -> String;
}

pub struct Author {
    firstname: String,
    lastname: String,
}

impl Display for Author {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {}", self.firstname, self.lastname)
    }
}

impl Author {
    pub fn new() -> Author {
        Author { firstname: String::new(), lastname: String::new() }
    }

    pub fn add_firstname(self, firstname: &str) -> Author {
        Author { firstname: format!("{}", firstname), lastname: self.lastname }
    }

    pub fn add_lastname(self, lastname: &str) -> Author {
        Author { firstname: self.firstname, lastname: format!("{}", lastname) }
    }
}

pub struct Post<'a> {
    content: String,
    author: &'a Author,
    history: Vec<String>,
}

pub struct PostDraft<'a> {
    content: Option<String>,
    author: Option<&'a Author>,
    history: Vec<String>,
}

pub struct PostPendingReview<'a> {
    content: Option<String>,
    author: Option<&'a Author>,
    history: Vec<String>,
}

impl<'a> Post<'a> {
    pub fn new() -> PostDraft<'a> {
        PostDraft {
            content: None,
            author: None,
            history: vec![String::from("Draft created")]
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn author(&self) -> String {
        format!("{}", &self.author)
    }
}

impl<'a> PostDraft<'a> {
    pub fn add_author(self, author: &'a Author) -> PostDraft<'a> {
        let new_history = format!("Author added: {}", author);
        PostDraft {
            content: self.content,
            author: Some(author),
            history: [&self.history[..], &vec![new_history]].concat()
        }
    }

    pub fn add_text(self, new: &str) -> PostDraft<'a> {
        let new_content = match self.content {
            Some(old) => format!("{}\n{}", old, new),
            None      => format!("{}", new),
        };
        let new_history = String::from("Text added");
        PostDraft {
            content: Some(new_content),
            author: self.author,
            history: [&self.history[..], &vec![new_history]].concat()
        }
    }

    pub fn request_review(self) -> PostPendingReview<'a> {
        let new_history = String::from("Review requested");
        PostPendingReview {
            content: self.content,
            author: self.author,
            history: [&self.history[..], &vec![new_history]].concat()
        }
    }
}

impl<'a> PostPendingReview<'a> {
    pub fn approve(self) -> Post<'a> {
        let new_history = String::from("Draft approved");
        Post {
            content: self.content.unwrap(),
            author: self.author.unwrap(),
            history: [&self.history[..], &vec![new_history]].concat()
        }
    }

    pub fn reject(self, message: &str) -> PostDraft<'a> {
        let new_history = format!("Draft rejected with message: '{}'", message);
        PostDraft {
            content: self.content,
            author: self.author,
            history: [&self.history[..], &vec![new_history]].concat()
        }
    }
}

impl<'a> PostHistory for Post<'a> {
    fn history(&self) -> String {
        let vstr: Vec<String> = self.history.iter().enumerate().map(|(i, h)| {
            format!("{} -> {}", i, h)
        }).collect();
        vstr.join("\n")
    }
}

impl<'a> PostHistory for PostDraft<'a> {
    fn history(&self) -> String {
        let vstr: Vec<String> = self.history.iter().enumerate().map(|(i, h)| {
            format!("{} -> {}", i, h)
        }).collect();
        vstr.join("\n")
    }
}

impl<'a> PostHistory for PostPendingReview<'a> {
    fn history(&self) -> String {
        let vstr: Vec<String> = self.history.iter().enumerate().map(|(i, h)| {
            format!("{} -> {}", i, h)
        }).collect();
        vstr.join("\n")
    }
}
