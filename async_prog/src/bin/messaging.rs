use std::time::Duration;

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();

        let first_batch_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let sender_fut = async move {
            let mut i = 0;
            while i < 10 {
                i += 1;
                if rand::random::<f64>() < 0.5 {
                    tx.send(String::from("random message")).unwrap();
                    trpl::sleep(Duration::from_millis(rand::random::<u64>() % 2000)).await;
                }
            }
        };

        let printer_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join!(first_batch_fut, printer_fut, sender_fut);
    });
}
