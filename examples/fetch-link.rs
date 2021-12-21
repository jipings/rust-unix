#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate select;
extern crate tokio;


use select::document::Document;
use select::predicate::Name;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

async fn run() -> Result<()> {
    let res = reqwest::get("https://www.rust-lang.org/en-US/").await?
        .text().await?;

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}