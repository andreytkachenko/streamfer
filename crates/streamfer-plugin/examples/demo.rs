#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut manager = streamfer_plugin::PluginManager::new();
    let plug = manager
        .load("/home/andrey/workspace/andreytkachenko/streamfer/target/release/")
        .await
        .unwrap();
}
