use std::ops::Range;

use rand::prelude::*;

pub async fn randn(range: Range<u32>) -> u32 {
    let mut rng = rand::thread_rng();
    let rand_n: u32 = rng.gen_range(range);
    rand_n
}
