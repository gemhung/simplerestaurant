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
            let sys = sysinfo::System::new_with_specifics(
                sysinfo::RefreshKind::nothing()
                    .with_memory(sysinfo::MemoryRefreshKind::nothing().with_ram()),
            );
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
    let cpu_usage = prometheus::PullingGauge::new(
        "cpu_usage",
        "Average cpu usage (1-100)",
        Box::new(move || {
            use sysinfo::{CpuRefreshKind, RefreshKind, System};
            let mut s = System::new_with_specifics(
                RefreshKind::nothing().with_cpu(CpuRefreshKind::nothing().with_cpu_usage()),
            );
            // Wait a bit because CPU usage is based on diff.
            std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
            // Refresh CPUs again to get actual value.
            s.refresh_cpu_usage();

            s.global_cpu_usage().trunc() as f64
        }),
    )
    .unwrap();
    prometheus
        .registry
        .register(Box::new(cpu_usage.clone()))
        .unwrap();

    prometheus
}
