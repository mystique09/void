use std::env;

use tokio::runtime::Runtime;

use void_infrastructure::bot;

pub fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let port = env::var("PORT").expect("PORT").parse::<u16>().expect("expecting a u16 type");
    let rt = Runtime::new().expect("tokio runtime");

    rt.block_on(bot::run(&database_url, port));
}
