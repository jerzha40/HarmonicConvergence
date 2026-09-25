
fn exponent(x: f64) -> i32 {
    let bits = x.to_bits();
    let raw_exp = ((bits >> 52) & 0x7ff) as i32;
    raw_exp - 1023
}

fn main() {
    let mut sum: f64 = 0.0;
    let mut n: u64 = 1;

    // 前面只有几百万步，直接真的算。
    // 一直算到进入 [16, 32)
    while sum < 16.0 {
        sum += 1.0 / n as f64;
        n += 1;
    }

    println!("entered [16, 32):");
    println!("last n = {}", n - 1);
    println!("sum    = {:.17}", sum);

    loop {
        let e = exponent(sum);

        // 当前 binade 的 ULP
        let h = 2.0_f64.powi(e - 52);

        // sum = j * h
        let mut j = (sum / h).round() as u64;

        // 下一个 binade 边界对应的格点编号
        let upper_j: u64 = 1_u64 << 53;

        // A = 1/h = 2^(52-e)
        let a: u64 = 1_u64 << (52 - e);

        // 用 2A/n 判断 round 到多少格
        let b: u64 = a << 1;

        loop {
            // 精确的 half-ULP tie 点单独真实计算
            if n == b {
                let next = sum + 1.0 / n as f64;

                if next == sum {
                    println!("\nNumerically stopped.");
                    println!("n   = {}", n);
                    println!("sum = {:.17}", sum);
                    println!("1/n = {:.17e}", 1.0 / n as f64);
                    return;
                }

                sum = next;
                n += 1;
                break;
            }

            let t = b / n;

            // 已经严格小于 half ULP
            if t == 0 {
                println!("\nNumerically stopped.");
                println!("n   = {}", n);
                println!("sum = {:.17}", sum);
                println!("1/n = {:.17e}", 1.0 / n as f64);
                return;
            }

            // floor(B/n) = t 可以保持到这里
            let mut r = b / t;

            // 最后那个 exact tie 留下来单独处理
            if r >= b {
                r = b - 1;
            }

            // 这一整个区间每一步跳 k 个格子
            let k = (t + 1) / 2;

            let count = r - n + 1;

            // 离下一个 2 的幂边界还有多少格
            let need = upper_j - j;

            // 最多批量执行多少步而不越过 binade
            let before_boundary = (need - 1) / k;

            if count <= before_boundary {
                // 整个区间直接一次跳完
                j += count * k;
                n = r + 1;
                sum = j as f64 * h;
            } else {
                // 先跳到边界前
                if before_boundary > 0 {
                    j += before_boundary * k;
                    n += before_boundary;
                    sum = j as f64 * h;
                }

                // 跨 binade 的这一项真正做一次 f64 加法
                let next = sum + 1.0 / n as f64;

                if next == sum {
                    println!("\nNumerically stopped.");
                    println!("n   = {}", n);
                    println!("sum = {:.17}", sum);
                    println!("1/n = {:.17e}", 1.0 / n as f64);
                    return;
                }

                sum = next;
                n += 1;

                // ULP 已经可能变化，重新计算
                break;
            }
        }
    }
}