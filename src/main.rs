use void::bootstrap::application::Application;

#[tokio::main]
async fn main() {
    dotenv::from_filename(".sample.env").ok();
    tracing_subscriber::fmt::init();

    let mut app = Application::new().await;
    app.start().await;
}
