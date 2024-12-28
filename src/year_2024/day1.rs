
type Input = (Vec<i32>, Vec<i32>);

fn total_distance(input: Input) -> i32 {
    let mut sorted1 = input.0.to_vec();
    let mut sorted2 = input.1.to_vec();
    sorted1.sort();
    sorted2.sort();
    sorted1.into_iter().zip(sorted2.into_iter()).map(|(x, y)| (x - y).abs()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static::lazy_static! {
        static ref SAMPLE: Input = parse_input(
            r#"
            3 4
            4 3
            2 5
            1 3
            3 9
            3 3"#
        );

        static ref INPUT: Input = read_input();
    }

    fn parse_line(line: &str) -> (i32, i32) {
        let nums: Vec<i32> = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        (nums[0], nums[1])
    }

    fn parse_input(file: &str) -> Input
    {
        file
            .lines()
            .filter(|x| !x.is_empty())
            .map(parse_line)
            .unzip()
    }

    fn read_input() -> Input {
        let contents = std::fs::read_to_string("input/2024/day1.txt").unwrap();
        parse_input(&*contents)
    }

    mod parse_input {
        use super::*;

        #[test]
        fn it_parses_the_sample_input() {
            let result = parse_input(
                r#"3 4
                   8 5"#
            );
            let expected = (vec![3, 8], vec![4, 5]);
            assert_eq!(expected, result);
        }

        #[test]
        fn it_parses_the_input() {
            let (vec1, vec2) = read_input();
            assert_eq!(&vec1[..5], &[76569, 38663, 60350, 35330, 88681]);
            assert_eq!(&vec2[..5], &[66648, 66530, 60777, 13469, 66648]);
        }
    }
    mod total_distance {
        use super::*;

        mod when_using_the_sample {
            use super::*;

            #[test]
            fn it_returns_the_expected_distance() {
                let sample: Input = parse_input(r#"3 4
                                                   4 3
                                                   2 5
                                                   1 3
                                                   3 9
                                                   3 3"#);
                let result = total_distance(sample);
                assert_eq!(result, 11);
            }
        }

        mod when_using_the_input {
            use super::*;

            #[test]
            fn it_returns_the_expected_distance() {
                let input = read_input();
                let result = total_distance(input);
                assert_eq!(result, 3246517);
            }
        }
    }

}
