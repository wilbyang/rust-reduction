use sysinfo::{CpuExt, System, SystemExt};
use axum::extract::ws::{Message, WebSocketUpgrade};
use axum::response::IntoResponse;

pub async fn cpuinfo(ws: WebSocketUpgrade)-> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        let mut system = System::new_all();
        loop {
            interval.tick().await;
            system.refresh_all();
            // get cpu usage for each core
            // let cpu_usag//e = system.get_processors().iter().map(|p| p.get_cpu_usage()).collect::<Vec<f32>>();
            // get total cpu usage
            let cpu_metrics = system.cpus().iter().map(|c| c.cpu_usage()).collect::<Vec<f32>>();
            // send as json
            let json = serde_json::to_string(&cpu_metrics).unwrap();
            socket.send(Message::from(json)).await.unwrap();
            //sleep for a while
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    })

}