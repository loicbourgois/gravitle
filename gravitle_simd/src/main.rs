#![feature(portable_simd)]
#![feature(test)]
extern crate test;
use test::Bencher;
use std::simd::f32x16;


fn add (a: &mut f32x16, b: & f32x16) {
    *a = *a + b;
}


fn add2 (a: &mut [f32], b: & [f32] ) {
    for i in 0..16 {
        a[i] = a[i] + b [i];
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_add(bencher: &mut Bencher) {
        let mut a = f32x16::from_array([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let b = f32x16::from_array([1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
        bencher.iter(|| {
            add(&mut a, &b)
        });
    }

    #[bench]
    fn bench_add2(bencher: &mut Bencher) {
        let mut a = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let b = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        bencher.iter(|| {
            add2(&mut a, &b)
        });
    }
}


fn main() {
}
