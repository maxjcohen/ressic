use ressic::generator::Rss20;

#[test]
fn test_generate() {
    super::test_generate(&Rss20::new());
}
