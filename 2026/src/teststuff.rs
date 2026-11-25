#[macro_export]
macro_rules! boilerplate {
    (
        $($test_input: ident == $test_input_text: literal),+
        for tests: {
            $($part: ident: { $($input: ident => $expected: expr),+ $(,)? }),* $(,)?
        },
        bench1 == $answer1: expr,
        bench2 == $answer2: expr $(,)?
    ) => {
    fn main() {
        let raw_input = read_input(DAY, std::env::args().nth(1).as_deref());
        let (input, time) = timed(|| parse_input(&raw_input));
        println!("Parsed in {time:?}");
        let (answer, time) = timed(|| part1(&input));
        println!("Part 1: {answer} ({time:?})");
        let (answer, time) = timed(|| part2(&input));
        println!("Part 2: {answer} ({time:?})");
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use $crate::*;

        $(const $test_input: &str = $test_input_text;)+

        $($(paste::paste! {
            #[test]
            fn [<$part _test_ $input:lower>]() {
                assert_eq!($part(&parse_input($input)), $expected);
            }
        })+)*
        bench!(part1 == $answer1);
        bench!(part2 == $answer2);
        #[bench]
        fn bench_input_parsing(b: &mut test::Bencher) {
            let raw = read_input(DAY, None);
            b.iter(|| parse_input(test::black_box(&raw)));
        }
    }
    }
}

#[macro_export]
macro_rules! bench {
    ($part: ident == $expected: expr) => {
        paste::paste! {
            #[bench]
            fn [<$part _bench>](b: &mut test::Bencher) {
                let raw = read_input(DAY, None);
                let input = parse_input(&raw);
                b.iter(|| assert_eq!($part(test::black_box(&input)), $expected));
            }
        }
    };
}
