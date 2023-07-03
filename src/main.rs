use std::error::Error;

use ureq;
use serde::{Serialize,Deserialize};
use serde_json;

#[derive(Serialize , Deserialize,Debug)]
struct Article{
    title : String,
    url : String,
}

#[derive(Serialize,Deserialize,Debug)]
struct NewsApi {
    status : String,
    totalResults : u32,
    articles : Vec<Article>,
    
}



fn get_article(url:&str) -> Result<Vec<Article>,Box<dyn Error>>{
    let resp =ureq::get(url).call()?.into_string()?;
    //Ok(resp)
    // conver the string respo to json
     let articales:NewsApi = serde_json::from_str(&resp)?;    
     Ok(articales.articles)
}

fn render_article(articles:&Vec<Article>){
    
    for article in articles {
    println!("title : {}",article.title);
    println!("url : {}",article.url);
    println!("---");
    }
    
}



fn main()-> Result<(),Box<dyn Error>> {
    let url ="https://newsapi.org/v2/top-headlines?country=us&category=business&apiKey=6b14d54a26444a22ba0be335d6fd4c6d";
    let articles = get_article(&url)?;
    render_article(&articles);

  
    // println!("{:#?}",article);
    // dbg!(article);
    Ok(())
}
