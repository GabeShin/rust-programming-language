use traits::{Display, NewsArticle, Summary, Tweet};

/* Using Traits as Parameters */
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/* Trait Bound Syntax
- This is much more verbose.
*/
fn notify_2<T: Summary>(item: &T) {
    println!("Breaking news again! {}", item.summarize());
}

/* Multiple Trait Bounds with the + Syntax */
pub fn notify_multiple(item: &(impl Summary + Display)) {}
pub fn notify_multiple_2<T: Summary + Display>(item: &T) {}

/* Clearer Trait Bounds with where Clauses
- using too many trait bound make signature difficult to read.
- using where clause could help with readability
*/
fn function_with_too_many_trait_bounds<T: Display + Clone, U: Clone + Summary>(t: &T, u: &U) {}
fn function_using_where_clause<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Summary,
{
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify(&article);
    notify_2(&article);
}
