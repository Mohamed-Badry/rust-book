use std::{time::Duration};

use trpl::Either;

fn main() {
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_millis(1500)).await;
            "I finished!"
        };

        match timeout(slow, Duration::from_millis(2000)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }

    });
}

async fn timeout<F: Future>(
    future: F,
    max_duration: Duration
) -> Result<F::Output, Duration> {

    let winner = trpl::race(future, trpl::sleep(max_duration));
    match winner.await {
        Either::Left(result) => Ok(result),
        Either::Right(_) => Err(max_duration),
    }
}
