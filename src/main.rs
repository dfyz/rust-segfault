fn do_test(x: usize) {
    let mut z = Vec::new();
    for _ in 0..1 {
        for y in 0..x {
            for _ in 0..1 {
                z.extend(std::iter::repeat(0).take(x));
                let a = y * x;
                let b = (y + 1) * x - 1;
                let slice = &mut z[a..b];
                slice[1 << 24] += 1;
            }
        }
    }
}

fn main() {
    do_test(1);
    do_test(2);
}
