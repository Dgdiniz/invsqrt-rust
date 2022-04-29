fn main() {
    println!("{}", invsqrt(16.0));
}

fn invsqrt(x: f32) -> f32 {
    let xhalf: f32 = 0.5 * x;
    let mut i = x.to_bits();

    i = 0x5f3759df - (i >> 1);

    let mut y = f32::from_bits(i);
    y = y * (1.5f32 - xhalf * y * y);

    return y;
}
