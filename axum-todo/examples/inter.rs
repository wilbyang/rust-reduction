use rand::Rng;
use std::iter::repeat;

fn main() {
    println!("Compute π using the Monte Carlo method.");
    for estimate in compute_pi().take(100) {
        println!("π ≅ {}", estimate);
    }
}

/// Generates a stream of increasingly accurate estimates of π.
fn compute_pi() -> impl Iterator<Item = f64> {
    let mut total = 0;
    let mut count = 0;
    repeat(()).map(move |_| {
        let points = generate_random().take(100000);
        let inside = points.filter(|p| p.is_inside_unit_circle());

        total += 100000;
        count += inside.count();
        let ratio = count as f64 / total as f64;

        // Area of a circle is A = π⋅r², therefore π = A/r².
        // So, when given random points with x ∈ <0,1>,
        // y ∈ <0,1>, the ratio of those inside a unit circle
        // should approach π / 4. Therefore, the value of π
        // should be:
        ratio * 4.0
    })
}

fn generate_random() -> impl Iterator<Item = Point> {
    let mut rng = rand::thread_rng();
    repeat(()).map(move |_| Point(rng.gen(), rng.gen()))
}

#[derive(Debug)]
struct Point(f64, f64);

impl Point {
    fn is_inside_unit_circle(&self) -> bool {
        self.0 * self.0 + self.1 * self.1 <= 1.0
    }
}
