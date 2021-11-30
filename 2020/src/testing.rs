#[macro_export]
macro_rules! bench {
    ($part: ident ($($param: expr),*) == $expected:expr) => {
        paste! {
            #[bench]
            fn [<$part _bench>](b: &mut test::Bencher) {
                let input = read_input();
                b.iter(|| assert_eq!($part(black_box(&input)$(, $param)*), $expected));
            }
        }
    };
}

#[macro_export]
macro_rules! bench_input {
    ($fn:ident()) => {
        #[bench]
        fn bench_input_parsing(b: &mut test::Bencher) {
            b.iter(|| black_box(read_input()));
        }
    };
}
