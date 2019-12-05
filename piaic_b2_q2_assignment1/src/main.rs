pub struct NewsArticle {
    author:String,
    content:String
}
pub struct Tweet {
    username:String,
    content:String
}
pub trait Summary {
    fn summarize(&self)->String;
}
impl Summary for NewsArticle{
    fn summarize(&self)->String {
        let info_for_newsarticle = format!("\nAuthor  : {}\nContent : {}",self.author,self.content);
        info_for_newsarticle   
    }
}
impl Summary for Tweet{
    fn summarize(&self)->String {
        let info_for_tweet = format!("\nUsername : {}\nContent  : {}",self.username,self.content);
        info_for_tweet
    }
}
fn main()
{
    let news_article_01 = NewsArticle{
        author:"Mark".to_string(),
        content:"Hello World".to_string()
    };
    let tweet_01 = Tweet{
        username:"John".to_string(),
        content:"Hello World".to_string()
    };
    println!("{}",news_article_01.summarize());
    println!("{}",tweet_01.summarize());
}
