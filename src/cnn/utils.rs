use rand;

//returns a random float in the range -1 < n < 1
pub fn random_clamped() -> f32{
    rand::random::<f32>() - rand::random::<f32>()
}

pub fn rand_float() -> f32{
    rand::random::<f32>()
}