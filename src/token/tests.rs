use super::*;
use rstest::rstest;

#[rstest]
#[case("", &[])]
#[case(" ", &[" "])]
#[case("    ",  &["    "])]
#[case("apa bepa cepa",  &["apa ", "bepa ", "cepa "])]
#[case("  apa bepa cepa",  &["  apa ", "bepa ", "cepa "])]
#[case("  apa bepa cepa  ",  &["  apa ", "bepa ", "cepa  "])]
fn test_tokenize(#[case] input: &str, #[case] expected: &[&str]) {
    let actual = tokenize(input);

    assert_eq!(actual, expected);
}
