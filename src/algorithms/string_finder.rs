pub trait StringFinder {
    fn index_of_lcs(&self, match_with: &str) -> Vec<&str>;
    fn matches_longer_than(&self, match_with: &str, threshold: usize) -> bool;
}