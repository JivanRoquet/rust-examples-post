mod lib;

use lib::{Post, Author};

fn main() {
    let mc = Author::new().add_firstname("Michael").add_lastname("Cane");
    let pmc = Post::new();
    let pmc = pmc.add_text("Hello, this is Michael.");
    let pmc = pmc.add_text("Listen to my little story.");
    let pmc = pmc.request_review();
    let pmc = pmc.reject("Please add a story");
    let pmc = pmc.add_text("Okay, I'll tell you next time actually.");
    let pmc = pmc.request_review();
    let pmc = pmc.reject("Please add an author");
    let pmc = pmc.add_author(&mc);
    let pmc = pmc.request_review();
    let pmc = pmc.approve();

    let jj  = Author::new().add_firstname("Jesse").add_lastname("James");
    let pjj = Post::new();
    let pjj = pjj.add_author(&jj);
    let pjj = pjj.add_text("Hi, it's Jesse speaking.");
    let pjj = pjj.add_text("Do you want to know a good one?");
    let pjj = pjj.add_text("Disclaimer: I'm not that good at telling jokes.");
    let pjj = pjj.request_review();
    let pjj = pjj.approve();

    let mj  = Author::new().add_firstname("Michael").add_lastname("Jackson");
    let pmj = Post::new();
    let pmj = pmj.add_author(&mj);
    let pmj = pmj.add_text("Hey there it's Michael Jackson.");
    let pmj = pmj.add_text("Hey pretty baby with the high heels on");
    let pmj = pmj.add_text("You give me fever like I've never, ever known");
    let pmj = pmj.request_review();
    let pmj = pmj.approve();

    let posts = vec![pmc, pjj, pmj];
    for post in posts {
        println!("{}", post.history());
        println!("{}", post.content());
        println!("-- by {}", post.author());
        println!("");
    }
}
