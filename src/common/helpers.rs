use rand::Rng;

pub fn coinflip() -> bool
{
    let mut rng = rand::thread_rng();
    rng.gen::<f32>() >= 0.50
}
