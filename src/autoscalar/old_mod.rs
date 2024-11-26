use std::collections::VecDeque;
use chrono::{DateTime, Utc};
use std::rc::*;
pub mod proposal;

#[derive(Debug, Clone)]
pub struct CurrentMetrics {
    pub cpu_utilization:     VecDeque<f64>,
    pub ram_utilization:     VecDeque<f64>,
    pub client_count:        VecDeque<u32>,
    pub response_time_ms:    VecDeque<f64>,
    pub disk_bandwidth_mbps: VecDeque<f64>,
    pub network_latency_ms:  VecDeque<f64>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct HistoricalMetrics {
    pub cpu_utilization:     VecDeque<f64>,
    pub ram_utilization:     VecDeque<f64>,
    pub client_count:        VecDeque<u32>,
    pub response_time_ms:    VecDeque<f64>,
    pub disk_bandwidth_mbps: VecDeque<f64>,
    pub network_latency_ms:  VecDeque<f64>,
}

#[derive(Debug, Clone)]
pub struct InstanceResources {
    pub cpu_cores:    u32,
    pub ram_gb:       u32,
    pub disk_iops:    u32,
    pub network_mbps: u32,
    pub max_clients:  u32,
}

#[derive(Debug, Clone)]
pub struct ScalingConfig {
    pub min_instances:           u32,
    pub max_instances:           u32,
    pub cpu_threshold:           f64,
    pub ram_threshold:           f64,
    pub response_time_threshold: f64,
    pub scale_up_factor:         f64,
    pub scale_down_factor:       f64,
    pub cooldown_minutes:        u32,
}

impl Default for ScalingConfig {
    fn default() -> Self {
        ScalingConfig {
            min_instances:           1,
            max_instances:           100,
            cpu_threshold:           70.0,
            ram_threshold:           80.0,
            response_time_threshold: 500.0,
            scale_up_factor:         2.0,
            scale_down_factor:       0.7,
            cooldown_minutes:        5,
        }
    }
}

pub struct AutoScaler {
    config:            ScalingConfig,
    current_instances: u32,
    last_scale_time:   DateTime<Utc>,
}

impl AutoScaler {
    pub fn new(config: ScalingConfig, current_instances: u32) -> Self {
        AutoScaler {
            config,
            current_instances,
            last_scale_time: Utc::now(),
        }
    }

    pub fn calculate_target_instances(&self,
        current:            &CurrentMetrics,
        historical:         &HistoricalMetrics,
        instance_resources: &InstanceResources,
    ) -> u32 {
        if (Utc::now() - self.last_scale_time).num_minutes() < self.config.cooldown_minutes as i64 {
            println!("Cooldown period active. No scaling performed.");
            return self.current_instances;
        }

        let cpu_load =                self.calculate_load_factor(&current.cpu_utilization, self.config.cpu_threshold);
        let ram_load =                self.calculate_load_factor(&current.ram_utilization, self.config.ram_threshold);
        let client_load =             self.calculate_client_load_factor(&current.client_count, instance_resources.max_clients);
        let response_time_load =      self.calculate_response_time_factor(&current.response_time_ms);
        let historical_cpu_trend =    self.calculate_trend(&historical.cpu_utilization);
        let historical_client_trend = self.calculate_trend_u32(&historical.client_count);

        let scaling_factor = self.determine_scaling_factor(
            cpu_load,
            ram_load,
            response_time_load,
            client_load,
            historical_cpu_trend,
            historical_client_trend,
        );

        let target_instances =  (self.current_instances as f64 * scaling_factor).round() as u32;
        let bounded_instances = self.bound_instances(target_instances);

        println!("Scaling decision: current_instances={}, target_instances={}, bounded_instances={}",
            self.current_instances, target_instances, bounded_instances);

        bounded_instances
    }

    fn calculate_load_factor(&self, measurements: &VecDeque<f64>, threshold: f64) -> f64 {
        if measurements.is_empty() {
            return 1.0;
        }

        let avg =  measurements.iter().sum::<f64>() / measurements.len() as f64;
        let peak = measurements.iter().fold(0.0_f64, |a, &b| a.max(b));
        
        let normalized_avg =  avg / threshold;
        let normalized_peak = peak / threshold;
        
        let load = 0.6 * normalized_avg + 0.4 * normalized_peak;
        let final_load = if load > 1.0 {
            load * 1.5  // Amplify when over threshold
        } else {
            load
        };

        println!("Load factor: avg={}, peak={}, normalized_avg={}, normalized_peak={}, final_load={}",
            avg, peak, normalized_avg, normalized_peak, final_load);

        final_load
    }

    fn calculate_response_time_factor(&self, measurements: &VecDeque<f64>) -> f64 {
        if measurements.is_empty() {
            return 1.0;
        }

        let avg =        measurements.iter().sum::<f64>() / measurements.len() as f64;
        let normalized = avg / self.config.response_time_threshold;
        
        let final_factor = if normalized > 1.0 {
            normalized * 1.3
        } else {
            normalized
        };

        println!("Response time factor: avg={}, normalized={}, final_factor={}",
            avg, normalized, final_factor);

        final_factor
    }

    fn calculate_client_load_factor(&self, clients: &VecDeque<u32>, max_clients_per_instance: u32) -> f64 {
        if clients.is_empty() {
            return 1.0;
        }

        let avg_clients =    clients.iter().sum::<u32>() as f64 / clients.len() as f64;
        let total_capacity = self.current_instances as f64 * max_clients_per_instance as f64;
        let load = avg_clients / total_capacity;
        
        let final_load = if load > 0.8 {
            load * 1.4
        } else {
            load
        };

        println!("Client load factor: avg_clients={}, total_capacity={}, load={}, final_load={}",
            avg_clients, total_capacity, load, final_load);

        final_load
    }

    fn calculate_trend(&self, historical_data: &VecDeque<f64>) -> f64 {
        if historical_data.len() < 288 {  // At least 24 days of data
            return 0.0;
        }

        let recent_avg = historical_data.iter()
            .rev()
            .take(144)
            .sum::<f64>() / 144.0;
        
        let old_avg = historical_data.iter()
            .rev()
            .skip(144)
            .take(144)
            .sum::<f64>() / 144.0;

        let trend = ((recent_avg - old_avg) / old_avg).clamp(-1.0, 1.0);

        println!("Trend: recent_avg={}, old_avg={}, trend={}",
            recent_avg, old_avg, trend);

        trend
    }

    fn calculate_trend_u32(&self, historical_data: &VecDeque<u32>) -> f64 {
        if historical_data.len() < 288 {
            return 0.0;
        }

        let recent_avg = historical_data.iter()
            .rev()
            .take(144)
            .sum::<u32>() as f64 / 144.0;
        
        let old_avg = historical_data.iter()
            .rev()
            .skip(144)
            .take(144)
            .sum::<u32>() as f64 / 144.0;

        let trend = ((recent_avg - old_avg) / old_avg).clamp(-1.0, 1.0);

        println!("Trend (u32): recent_avg={}, old_avg={}, trend={}",
            recent_avg, old_avg, trend);

        trend
    }

    fn determine_scaling_factor(
        &self,
        cpu_load:                f64,
        ram_load:                f64,
        response_time_load:      f64,
        client_load:             f64,
        historical_cpu_trend:    f64,
        historical_client_trend: f64,
    ) -> f64 {
        let weights = [
            (cpu_load, 0.35),          // CPU weight
            (ram_load, 0.25),          // RAM weight
            (response_time_load, 0.2),  // Response time weight
            (client_load, 0.1),        // Current client load weight
            (1.0 + historical_cpu_trend, 0.05),  // Historical CPU trend weight
            (1.0 + historical_client_trend, 0.05), // Historical client trend weight
        ];

        let combined = weights.iter()
            .map(|(factor, weight)| factor * weight)
            .sum::<f64>();

        let scaling_factor = if combined > 1.2 {
            self.config.scale_up_factor
        } else if combined > 1.0 {
            1.5  // Moderate scale up
        } else if combined < 0.6 {
            self.config.scale_down_factor
        } else {
            1.0
        };

        println!("Scaling factor: cpu_load={}, ram_load={}, response_time_load={}, client_load={}, historical_cpu_trend={}, historical_client_trend={}, combined={}, scaling_factor={}",
            cpu_load, ram_load, response_time_load, client_load, historical_cpu_trend, historical_client_trend, combined, scaling_factor);

        scaling_factor
    }

    fn bound_instances(&self, instances: u32) -> u32 {
        let min_change = (self.current_instances as f64 * 0.2).ceil() as u32;  // Minimum 20% change
        let bounded_instances = if instances > self.current_instances {
            let increase = (instances - self.current_instances).max(min_change);
            (self.current_instances + increase).clamp(self.config.min_instances, self.config.max_instances)
        } else if instances < self.current_instances {
            let decrease = (self.current_instances - instances).max(min_change);
            (self.current_instances - decrease).clamp(self.config.min_instances, self.config.max_instances)
        } else {
            self.current_instances
        };

        println!("Bound instances: requested_instances={}, min_change={}, bounded_instances={}",
            instances, min_change, bounded_instances);

        bounded_instances
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::*;

    #[test]
    fn test_autoscaler_basic() {
        let config =  Rc::new(ScalingConfig::default());
        let instance_resources = InstanceResources {
            cpu_cores:    4,
            ram_gb:       16,
            disk_iops:    1000,
            network_mbps: 1000,
            max_clients:  100,
        };

        // Initialize current metrics with high load
        let mut current_metrics = CurrentMetrics {
            cpu_utilization:     VecDeque::new(),
            ram_utilization:     VecDeque::new(),
            client_count:        VecDeque::new(),
            response_time_ms:    VecDeque::new(),
            disk_bandwidth_mbps: VecDeque::new(),
            network_latency_ms:  VecDeque::new(),
            timestamp:           Utc::now(),
        };

        // Simulate very high load
        for _ in 0..20 {
            current_metrics.cpu_utilization.push_back(90.0);  // High CPU usage
            current_metrics.ram_utilization.push_back(85.0);  // High RAM usage
            current_metrics.client_count.push_back(95);       // Near max clients
            current_metrics.response_time_ms.push_back(450.0); // High response time
            current_metrics.disk_bandwidth_mbps.push_back(80.0);
            current_metrics.network_latency_ms.push_back(30.0);
        }

        // Initialize historical metrics showing increasing load trend
        let mut historical_metrics = HistoricalMetrics {
            cpu_utilization:     VecDeque::with_capacity(1080),
            ram_utilization:     VecDeque::with_capacity(1080),
            client_count:        VecDeque::with_capacity(1080),
            response_time_ms:    VecDeque::with_capacity(1080),
            disk_bandwidth_mbps: VecDeque::with_capacity(1080),
            network_latency_ms:  VecDeque::with_capacity(1080),
        };

        // Fill historical data showing increasing load
        for i in 0..1080 {
            let trend_factor = 1.0 + (i as f64 / 1080.0) * 0.5; // 50% increase over time
            historical_metrics.cpu_utilization.push_back(75.0 * trend_factor);
            historical_metrics.ram_utilization.push_back(70.0 * trend_factor);
            historical_metrics.client_count.push_back((80.0 * trend_factor) as u32);
            historical_metrics.response_time_ms.push_back(200.0 * trend_factor);
            historical_metrics.disk_bandwidth_mbps.push_back(50.0 * trend_factor);
            historical_metrics.network_latency_ms.push_back(20.0 * trend_factor);
        }

        let mut scaler = AutoScaler::new((*config).clone(), 4);
        scaler.last_scale_time = Utc::now() - chrono::Duration::minutes(config.cooldown_minutes as i64 + 1);
        let new_instances = scaler.calculate_target_instances(
            &current_metrics,
            &historical_metrics,
            &instance_resources,
        );

        // With high load and increasing trends, we expect significant scaling
        assert!(new_instances > 4, "Expected scaling up from 4 instances, got {}", new_instances);
    }
}
