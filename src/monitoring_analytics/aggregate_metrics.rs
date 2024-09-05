use crate::monitoring_analytics::performance_metrics::PerformanceMetrics;
use crate::monitoring_analytics::aggregated_metrics::AggregatedMetrics;

pub fn aggregate_metrics(metrics_list: Vec<PerformanceMetrics>) -> AggregatedMetrics {
    let mut aggregated_metrics = AggregatedMetrics::new();

    for metrics in metrics_list {
        aggregated_metrics.add_metrics(&metrics);
    }

    aggregated_metrics
}
