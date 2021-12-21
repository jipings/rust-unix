use error_chain::error_chain;
use reqwest::{StatusCode};
use select::document::Document;
use select::predicate::Name;
use std::{collections::HashSet};
use url::{Position, Url};


error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
        UrlParseError(url::ParseError);
        JoinError(tokio::task::JoinError);
    }
  }

/**
 * Check a webapp for broken links
 * Call `get_base_url` to retrieve the base URL. If the document has a base tag, get the href `attr` from base tag.
 * `Position::BeforePath` of the original URL acts as a default. 
 * 
 * Iterates through links in the document and creates a `tokio::spawn` task that will parse an individual
 * link with `url::ParseOptions` and `Url::parse`. The task makes a request to the links with reqwest and verifies 
 * `StatusCode`. Then the tasks `await` completion before ending the program.
*/
async fn get_base_url(url: &Url, doc: &Document) -> Result<Url> {
    let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).nth(0);
    let base_url = 
    base_tag_href.map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)?;

    Ok(base_url)
}

async fn check_link(url: &Url) -> Result<bool> {
    let res = reqwest::get(url.as_ref()).await?;
    Ok(res.status() != StatusCode::NOT_FOUND)
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = Url::parse("https://www.rust-lang.org/en-US/")?;
    let res = reqwest::get(url.as_ref()).await?.text().await?;
    let document = Document::from(res.as_str());
    let base_url = get_base_url(&url, &document).await?;
    let base_parser = Url::options().base_url(Some(&base_url));
    let links: HashSet<Url> = document
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter_map(|link| base_parser.parse(link).ok())
        .collect();
    let mut tasks = vec![];

    println!("{:?}", links);

    for link in links {
        tasks.push(tokio::spawn(async move {
            if check_link(&link).await.unwrap() {
                println!("{} is OK", link);
            } else {
                println!("{} is Broken", link);
            }
        }));
    }

    for task in tasks {
        task.await?
    }

    Ok(())
}

// TODO timeout ?