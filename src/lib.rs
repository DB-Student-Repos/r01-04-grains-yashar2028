pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64");
    }

   let mut n = 1;
   for i in 1..=s-1 {
       n *= 2;
    }
    n
}

pub fn total() -> u64 {
    let total = (1 .. 65).map(square).sum();
        total


}
