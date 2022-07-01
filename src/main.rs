mod random_price_fetcher;

use std::thread;
use std::time::Duration;
use notify_rust::Notification;
use crate::random_price_fetcher::fetch_price_random;


fn main() {
    start_program(fetch_price_random,
                  vec![notify_to_console, notify_to_desktop_notifications])
}

fn start_program(price_fetcher: impl Fn() -> i64, notifiers: Vec<fn(i64)>) {
    let mut highest_price: i64 = 0;
    loop {
        thread::sleep(Duration::from_millis(1000));

        let result = price_fetcher();
        if result > highest_price {
            highest_price = result;

            //notify
            notifiers.iter().for_each(|o| o(result));
        }
    }

}

pub fn notify_to_console(number: i64) {
    println!("New price: {}", number)
}

pub fn notify_to_desktop_notifications(number: i64) {
    Notification::new()
        .summary("New Price")
        .appname("PriceNotifier")
        .body(&*format!("new price: {}", number))
        .show().unwrap();
}