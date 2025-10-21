use prometheus::{HistogramVec, CounterVec, Gauge, register_histogram_vec, register_counter_vec, register_gauge};
use lazy_static::lazy_static;

lazy_static! {
    // Latence par endpoint
    pub static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "http_request_duration_seconds",
        "Durée des requêtes HTTP",
        &["method", "endpoint"]
    ).unwrap();

    // Trafic RPS
    pub static ref HTTP_REQ_COUNTER: CounterVec = register_counter_vec!(
        "http_requests_total",
        "Nombre total de requêtes HTTP",
        &["method", "endpoint"]
    ).unwrap();

    // Erreurs 4xx/5xx
    pub static ref HTTP_ERR_COUNTER: CounterVec = register_counter_vec!(
        "http_errors_total",
        "Nombre total d'erreurs HTTP",
        &["method", "endpoint", "status"]
    ).unwrap();

    // Saturation CPU/RAM
    pub static ref SYSTEM_CPU: Gauge = register_gauge!(
        "system_cpu_percent",
        "CPU usage (%)"
    ).unwrap();
    pub static ref SYSTEM_RAM: Gauge = register_gauge!(
        "system_ram_bytes",
        "RAM usage (bytes)"
    ).unwrap();
}
