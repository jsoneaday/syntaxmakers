pub fn get_random_no_from_range(start: usize, end_non_inclusive: usize) -> usize {
    use rand::Rng;
    
    rand::thread_rng().gen_range(start..end_non_inclusive)
}