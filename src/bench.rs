extern crate test;


#[cfg(test)]
mod tests {
    use crate::test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..100 {
                test::black_box(test());
            }
        });
    }
}
