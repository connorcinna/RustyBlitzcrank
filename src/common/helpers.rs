use rand::Rng;

pub fn coinflip() -> bool
{
    rand::rng().random_bool(1.0 / 2.0)
}
