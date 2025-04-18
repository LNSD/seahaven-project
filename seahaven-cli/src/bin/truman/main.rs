mod cmd;
mod deps;

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();

    // Parse and run
    if let Err(err) = cmd::run().await {
        eprintln!("{err}");
        std::process::exit(err.code());
    }
}
