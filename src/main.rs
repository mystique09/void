use void::bootstrap::application::Application;

#[tokio::main]
async fn main() {
    let mut app = Application::new().await;
    app.start().await;
}
