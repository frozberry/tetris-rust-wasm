use rand::seq::SliceRandom;
#[derive(Clone, Copy)]
pub enum Shape {
    Q,
    I,
}

impl Shape {
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        *[Shape::Q, Shape::I].choose(&mut rng).unwrap()
    }
}
