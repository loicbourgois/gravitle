#[test]
mod test {
    use crate::wrap_around;
    fn approx_equal(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.00000001
    }

    fn test_wrap_around() {
        println!("boby");
        let a = Vector { x: 0.1, y: 0.09 };
        let b = Vector { x: 0.1015, y: 0.09 };
        let wa = wrap_around(&a, &b);
        let diam = 0.001 * 2.0;
        let diam_sqrd = diam * diam;
        println!("wa.d_sqrd: {}", wa.d_sqrd);
        println!("diam_sqrd: {}", diam_sqrd);
        println!("wa.d: {:?}", wa.d);
        println!("diam: {}", diam);
        println!("wa: {:?}", wa);
        assert!(approx_equal(a.x, wa.a.x));
        assert!(approx_equal(b.x, wa.b.x));
        assert!(approx_equal(a.y, wa.a.y));
        assert!(approx_equal(b.y, wa.b.y));
        // let a = [
        //     Vector {
        //         x: 0.9431783,
        //         y: 0.7500257,
        //     },
        //     Vector {
        //         x: 0.19544017,
        //         y: 0.49620646,
        //     },
        // ];
        // let b = wrap_around(&a[0], &a[1]);
        // assert!(b.d_sqrd.sqrt() < 0.37);

        // let a = [
        //     Vector {
        //         x: 0.7241143,
        //         y: 0.487625,
        //     },
        //     Vector {
        //         x: 0.22385669,
        //         y: 0.49002028,
        //     },
        // ];
        // let b = wrap_around(&a[0], &a[1]);
        // assert!(b.d_sqrd.sqrt() < 0.51);

        // let a = [
        //     Vector {
        //         x: 0.74952763,
        //         y: 0.35994428,
        //     },
        //     Vector {
        //         x: 0.00013560057,
        //         y: 0.60556775,
        //     },
        // ];
        // let b = wrap_around(&a[0], &a[1]);
        // println!("{}", b.d_sqrd.sqrt());
        // assert!(b.d_sqrd.sqrt() < 0.39);

        // let mut max_d: f32 = 0.0;
        // let mut rng = rand::thread_rng();
        // let mut ps_max = Vec::new();
        // let mut wp_max = WrapAroundResponse {
        //     a: Vector { x: 0.0, y: 0.0 },
        //     b: Vector { x: 0.0, y: 0.0 },
        //     d_sqrd: 0.0,
        // };
        // for _ in 0..10_000_000 {
        //     let mut ps = Vec::new();
        //     for _ in 0..2 {
        //         ps.push(Vector {
        //             x: rng.gen(),
        //             y: rng.gen(),
        //         })
        //     }
        //     let wp = wrap_around(&ps[0], &ps[1]);
        //     let d = wp.d_sqrd.sqrt();
        //     if d > max_d {
        //         ps_max = ps;
        //         wp_max = wp
        //     }
        //     max_d = max_d.max(d)
        // }
        // assert!(max_d < 0.5_f32.sqrt());
    }
}
