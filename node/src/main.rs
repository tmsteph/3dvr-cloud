use axum::{routing::{get, post}, Router, Json};
use serde::Serialize;
use std::{net::SocketAddr, sync::{Arc, atomic::{AtomicU64, Ordering}}};
use sysinfo::{System, SystemExt, CpuRefreshKind, Disks};
use tokio::sync::Mutex;
use anyhow::Result;

#[derive(Clone)]
struct AppState {
    start: std::time::Instant,
    heartbeat_count: Arc<AtomicU64>,
    sys: Arc<Mutex<System>>,
}

#[derive(Serialize)]
struct Health {
    ok: bool,
    uptime_seconds: f64,
    heartbeats: u64,
}

#[derive(Serialize)]
struct Resources {
    cpu_load_avg: f32,
    total_mem_mb: u64,
    free_mem_mb: u64,
    disk_free_gb: f64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let state = AppState {
        start: std::time::Instant::now(),
        heartbeat_count: Arc::new(AtomicU64::new(0)),
        sys: Arc::new(Mutex::new(sys)),
    };

    let app = Router::new()
        .route("/health", get({
            let state = state.clone();
            move || async move {
                let uptime = state.start.elapsed().as_secs_f64();
                let hb = state.heartbeat_count.load(Ordering::Relaxed);
                Json(Health { ok: true, uptime_seconds: uptime, heartbeats: hb })
            }
        }))
        .route("/resources", get({
            let state = state.clone();
            move || async move {
                let mut sys = state.sys.lock().await;
                sys.refresh_cpu_specifics(CpuRefreshKind::everything());
                sys.refresh_memory();
                sys.refresh_disks_list();
                sys.refresh_disks();

                let total = sys.total_memory() / 1024; // to MB
                let free = sys.available_memory() / 1024; // to MB

                let mut free_disk_gb = 0.0;
                let disks = Disks::new_with_refreshed_list();
                for d in disks.list() {
                    let avail = d.available_space() as f64 / (1024.0*1024.0*1024.0);
                    free_disk_gb += avail;
                }

                let load = sys.global_cpu_info().cpu_usage(); // 0..100

                Json(Resources {
                    cpu_load_avg: load,
                    total_mem_mb: total,
                    free_mem_mb: free,
                    disk_free_gb: free_disk_gb,
                })
            }
        }))
        .route("/peer/connect", post({
            let state = state.clone();
            move || async move {
                // TODO: accept JSON body with { addr, pubkey? } and persist in a peer-book.
                state.heartbeat_count.fetch_add(1, Ordering::Relaxed);
                Json(serde_json::json!({"ok": true, "msg": "peer recorded (mock)"}))
            }
        }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8787));
    println!("people-cloud node listening on http://{addr}");
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}