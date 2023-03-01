use super::random_number::randn;

pub const WORDLIST: &str = include_str!("wordlist.txt");
pub const BUMPS: &str = include_str!("bumps.txt");
pub const RESPONSES: &str = include_str!("response.txt");

pub async fn generate_random_bump() -> &'static str {
    let bumps = BUMPS.split('\n').collect::<Vec<&str>>();
    let i = randn(0..(bumps.len() as u32)).await;

    bumps[i as usize]
}

pub async fn generate_random_response() -> &'static str {
    let responses = RESPONSES.split('\n').collect::<Vec<&str>>();
    let i = randn(0..(responses.len() as u32)).await;

    responses[i as usize]
}
