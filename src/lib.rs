use std::fmt::{Display, Formatter, Result};

pub trait PostHistory {
    fn history(&self) -> String;
}

pub struct Author {
    firstname: String,
    lastname: String,
}

pub struct History {
    history: Vec<String>,
}

impl History {
    fn new() -> History {
        History { history: vec![] }
    }

    fn show(&self) -> String {
        let vstr: Vec<String> = self.history.iter().enumerate().map(|(i, h)| {
            format!("{} -> {}", i, h)
        }).collect();
        vstr.join("\n")
    }

    fn add(&self, new: String) -> History {
        let history = [&self.history[..], &vec![new]].concat();
        History { history: history }
    }
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
    history: History,
}

pub struct PostDraft<'a> {
    content: Option<String>,
    author: Option<&'a Author>,
    history: History,
}

pub struct PostPendingReview<'a> {
    content: Option<String>,
    author: Option<&'a Author>,
    history: History,
}

impl<'a> Post<'a> {
    pub fn new() -> PostDraft<'a> {
        PostDraft {
            content: None,
            author: None,
            history: History::new().add(String::from("Draft created"))
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
        PostDraft {
            content: self.content,
            author: Some(author),
            history: self.history.add(format!("Author added: {}", author))
        }
    }

    pub fn add_text(self, new: &str) -> PostDraft<'a> {
        let new_content = match self.content {
            Some(old) => format!("{}\n{}", old, new),
            None      => format!("{}", new),
        };
        PostDraft {
            content: Some(new_content),
            author: self.author,
            history: self.history.add(format!("{} characters of text added", new.len()))
        }
    }

    pub fn request_review(self) -> PostPendingReview<'a> {
        PostPendingReview {
            content: self.content,
            author: self.author,
            history: self.history.add(String::from("Review requested"))
        }
    }
}

impl<'a> PostPendingReview<'a> {
    pub fn approve(self) -> Post<'a> {
        Post {
            content: self.content.unwrap(),
            author: self.author.unwrap(),
            history: self.history.add(String::from("Draft approved"))
        }
    }

    pub fn reject(self, message: &str) -> PostDraft<'a> {
        PostDraft {
            content: self.content,
            author: self.author,
            history: self.history.add(format!("Draft rejected with message: '{}'", message))
        }
    }
}

impl<'a> PostHistory for Post<'a> {
    fn history(&self) -> String {
        self.history.show()
    }
}

impl<'a> PostHistory for PostDraft<'a> {
    fn history(&self) -> String {
        self.history.show()
    }
}

impl<'a> PostHistory for PostPendingReview<'a> {
    fn history(&self) -> String {
        self.history.show()
    }
}
