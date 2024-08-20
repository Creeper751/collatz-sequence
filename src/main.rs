use clap::{command, value_parser, Arg};

fn collatz_sequence(mut n: i32) -> i32 {
    let mut len: i32 = 1;
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
        len += 1;
    }
    len
}
fn main() {
    let match_result = command!()
        .arg(
            Arg::new("number")
                .required(true)
                .short('n')
                .long("number")
                .help("The number to calculate the Collatz Sequence")
                .value_parser(value_parser!(i32)),
        )
        .about("A simple program to calculate the Collatz Sequence")
        .arg_required_else_help(true)
        .author("Creeper751")
        .bin_name("collatz-sequence")
        .get_matches();

    let n: i32 = *match_result.get_one::<i32>("number").unwrap();
    println!(
        "The Collatz Sequence Result of the number {} is: {}",
        n,
        collatz_sequence(n)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_collatz_sequence() {
        assert_eq!(collatz_sequence(1), 1);
        assert_eq!(collatz_sequence(11), 15);
    }
}
