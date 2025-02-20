use rand::Rng;

pub fn generate_data(n: usize) -> Vec<(f32, f32)> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| {
            let x = rng.gen_range(0.0..10.0);
            let noise: f32 = rng.gen_range(-1.0..1.0); // Small noise
            let y = 2.0 * x + 1.0 + noise;
            (x, y)
        })
        .collect()
}

