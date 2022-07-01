use rand::Rng;

pub fn fetch_price_random() -> i64 {
    let mut rng = rand::thread_rng();

    return rng.gen_range(0..1000)
}