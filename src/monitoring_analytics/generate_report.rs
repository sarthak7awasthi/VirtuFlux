use crate::monitoring_analytics::aggregated_metrics::AggregatedMetrics;
use crate::monitoring_analytics::alert::Alert;
use crate::monitoring_analytics::performance_report::PerformanceReport;

pub fn generate_report(
    time_range: String,
    aggregated_metrics: AggregatedMetrics,
    triggered_alerts: Vec<Alert>,
) -> PerformanceReport {
    PerformanceReport::new(time_range, aggregated_metrics, triggered_alerts)
}
