use tokio::runtime::Runtime;

use void_infrastructure::{bot, logger};
use void_infrastructure::bot::get_env;

pub fn main() {
    let bot_env = get_env();
    let rt = Runtime::new().expect("tokio runtime");

    logger::init_default_logger();
    rt.block_on(bot::run(&bot_env.database_url, &bot_env));
}
