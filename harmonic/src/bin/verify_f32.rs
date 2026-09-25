fn naive_f32() -> (u64, f32, f32) {
    let mut sum: f32 = 0.0;
    let mut n: u64 = 1;

    loop {
        let term = 1.0_f32 / n as f32;
        let next = sum + term;

        if next == sum {
            return (n, sum, term);
        }

        sum = next;
        n += 1;
    }
}

fn from_grid_e3(j: u64) -> f32 {
    // In [8, 16), exponent e = 3.
    // A normal f32 has significand j / 2^23 with
    // 2^23 <= j < 2^24.
    assert!((1_u64 << 23) <= j && j < (1_u64 << 24));

    let raw_exp: u32 = (3 + 127) as u32; // e + bias
    let frac: u32 = (j - (1_u64 << 23)) as u32;

    f32::from_bits((raw_exp << 23) | frac)
}

fn grid_index_e3(x: f32) -> u64 {
    assert!(x >= 8.0 && x < 16.0);

    let bits = x.to_bits();
    let frac = bits & 0x7f_ffff;

    (1_u64 << 23) + frac as u64
}

fn accelerated_f32() -> (u64, f32, f32, u64) {
    let mut sum: f32 = 0.0;
    let mut n: u64 = 1;

    // First enter the final binade [8, 16) by doing the real recurrence.
    while sum < 8.0 {
        let term = 1.0_f32 / n as f32;
        let next = sum + term;

        if next == sum {
            return (n, sum, term, 0);
        }

        sum = next;
        n += 1;
    }

    println!("accelerator starts:");
    println!("  first n to process = {}", n);
    println!("  current sum        = {:.15}", sum);
    println!("  bits               = {:032b}", sum.to_bits());

    // In [8,16):
    // p = 24, e = 3
    // ULP h = 2^(3-23) = 2^-20
    // B = 2/h = 2^21
    const B: u64 = 1_u64 << 21;

    let mut j = grid_index_e3(sum);
    let mut blocks: u64 = 0;

    loop {
        // n = B means 1/n = h/2 exactly.
        // This is the only exact midpoint tie in this binade,
        // so let the real f32 addition perform ties-to-even.
        if n == B {
            let term = 1.0_f32 / n as f32;
            let next = sum + term;

            if next == sum {
                return (n, sum, term, blocks);
            }

            sum = next;
            n += 1;
            j = grid_index_e3(sum);
            continue;
        }

        // q = floor(B/n) = floor(2 * (1/n)/h)
        let q = B / n;

        // n > B => 1/n < h/2, so the next addition cannot move sum.
        if q == 0 {
            let term = 1.0_f32 / n as f32;
            let next = sum + term;

            assert_eq!(next.to_bits(), sum.to_bits());

            return (n, sum, term, blocks);
        }

        // floor(B/n) remains q through this whole interval.
        let mut r = B / q;

        // Keep the exact half-ULP tie n=B for the real f32 branch above.
        if r >= B {
            r = B - 1;
        }

        // Away from ties:
        // k = round((1/n)/h) = floor((q+1)/2)
        let k = (q + 1) / 2;

        let count = r - n + 1;

        // Apply count identical grid jumps at once.
        j += count * k;

        // The f32 trajectory never leaves [8,16) before it stops.
        assert!(j < (1_u64 << 24));

        sum = from_grid_e3(j);
        n = r + 1;
        blocks += 1;
    }
}

fn main() {
    let (naive_n, naive_sum, naive_term) = naive_f32();
    let (fast_n, fast_sum, fast_term, blocks) = accelerated_f32();

    println!();
    println!("naive:");
    println!("  n     = {}", naive_n);
    println!("  sum   = {:.15}", naive_sum);
    println!("  1/n   = {:.15e}", naive_term);
    println!("  bits  = {:032b}", naive_sum.to_bits());

    println!();
    println!("accelerated:");
    println!("  n     = {}", fast_n);
    println!("  sum   = {:.15}", fast_sum);
    println!("  1/n   = {:.15e}", fast_term);
    println!("  bits  = {:032b}", fast_sum.to_bits());
    println!("  blocks= {}", blocks);

    assert_eq!(naive_n, fast_n);
    assert_eq!(naive_sum.to_bits(), fast_sum.to_bits());
    assert_eq!(naive_term.to_bits(), fast_term.to_bits());

    println!();
    println!("PASS: naive and accelerated f32 end bit-for-bit identically.");
}
