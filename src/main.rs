use std::env;
use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json;
use ureq;

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct NewsApi {
    status: String,
    totalResults: u32,
    articles: Vec<Article>,
}

fn get_article(url: &str) -> Result<Vec<Article>, Box<dyn Error>> {
    let resp = ureq::get(url).call()?.into_string()?;
    //Ok(resp)
    // conver the string respo to json
    let articales: NewsApi = serde_json::from_str(&resp)?;
    Ok(articales.articles)
}

fn render_article(articles: &Vec<Article>) {
    for article in articles {
        println!("title : {}", article.title);
        println!("url : {}", article.url);
        println!("---");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("API_KEY").map_err(|_| "API_KEY environment variable not set")?;
    let url = format!(
        "https://newsapi.org/v2/top-headlines?country=us&category=business&apiKey={}",
        api_key
    );
    let articles = get_article(&url)?;
    render_article(&articles);

   
    // dbg!(article);
    Ok(())
}
