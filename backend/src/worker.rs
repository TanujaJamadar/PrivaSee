use crate::analyzer::{analyze_traffic, extract_domain};
use crate::types::{AppState, NodeMessage};
use socketioxide::extract::SocketRef;
use std::env;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

pub async fn run_node_worker(
    socket: SocketRef,
    target_url: String,
    state: Arc<AppState>,
) -> std::io::Result<()> {
    let main_domain = extract_domain(&target_url);

    let scanner_dir = env::var("SCANNER_DIR").unwrap_or_else(|_| "/app/backend".into());
    let scanner_js =
        env::var("SCANNER_JS").unwrap_or_else(|_| format!("{}/scanner.js", scanner_dir));

    let mut child = Command::new("node")
        .current_dir(&scanner_dir)
        .arg(&scanner_js)
        .arg(&target_url)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let stdout = child.stdout.take().expect("Failed to open stdout");
    let mut reader = BufReader::new(stdout).lines();

    while let Ok(Some(line)) = reader.next_line().await {
        if !line.trim().starts_with('{') {
            continue;
        }

        if let Ok(msg) = serde_json::from_str::<NodeMessage>(&line) {
            match msg {
                NodeMessage::Status { message } => {
                    let _ = socket.emit("status", &message);
                }
                NodeMessage::Error { message } => {
                    let _ = socket.emit("status", &format!("Error: {}", message));
                }
                NodeMessage::Traffic { data } => {
                    let update = analyze_traffic(&data, &main_domain, &state);
                    let _ = socket.emit("traffic-update", &update);
                }
            }
        }
    }

    let _ = child.wait().await?;
    Ok(())
}
