fn main() {
    let mut sum: f32 = 0.0;
    let mut n: u64 = 1;

    loop {
        let next = sum + 1.0 / n as f32;

        if next == sum {
            println!("Numerically stopped.");
            println!("n = {}", n);
            println!("sum = {}", sum);
            println!("1/n = {}", 1.0 / n as f32);
            break;
        }

        sum = next;
        n += 1;
    }
}
