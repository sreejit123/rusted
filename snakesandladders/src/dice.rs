use rand::Rng;

pub fn dice() -> i8 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(1, 6);
}