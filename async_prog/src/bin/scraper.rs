use trpl::{Either, Html};

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text =  trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}

fn main() {

    let mut urls = vec![];
    urls.push(String::from("https://doc.rust-lang.org/book"));
    urls.push(String::from("https://docs.rust-embedded.org/book"));
    
    trpl::run(async {
        let title_fut_1 = page_title(&urls[1]);
        let title_fut_2 = page_title(&urls[0]);

        let (url, maybe_title) =
            match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };
        
        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
    
}