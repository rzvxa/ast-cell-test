extern crate test;


#[cfg(test)]
mod tests {
    use crate::test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn type_of(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1000 {
                test::black_box(test());
            }
        });
    }
}
