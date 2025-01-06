use actix_web_prom::PrometheusMetricsBuilder;

pub fn prometheus() -> actix_web_prom::PrometheusMetrics {
    let prometheus = PrometheusMetricsBuilder::new("")
        .endpoint("/metrics")
        .exclude("/metrics")
        .build()
        .unwrap();
    let mem_usage = prometheus::PullingGauge::new(
        "mem_usage",
        "Average memory usage (1-100)",
        Box::new(move || {
            let mut sys = sysinfo::System::new();
            sys.refresh_memory_specifics(sysinfo::MemoryRefreshKind::nothing().with_ram());
            let used = sys.used_memory();
            let total = sys.total_memory();
            if total > 0 {
                f64::trunc((used as f64 / total as f64) * 100.0)
            } else {
                0.0
            }
        }),
    )
    .unwrap();
    prometheus
        .registry
        .register(Box::new(mem_usage.clone()))
        .unwrap();

    prometheus
}
