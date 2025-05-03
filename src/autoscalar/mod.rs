use std::sync::{ Arc, Mutex };
use std::collections::HashMap;
use std::time::Duration;
use anyhow::Result;

/// A struct that represents the metrics that can be gathered from an AppInstance
/// # Fields
///
/// * `cpu_load` - Generally applicable CPU load for any container (Gathered from the runtime if supported)
/// * `ram_pressure` - Generally applicable memory pressure for any container (Gathered from the runtime if supported)
/// * `ram_usage` - Generally applicable memory usage for any container (Gathered from the runtime if supported)
/// * `clients` - For more advanced services that are client-aware
/// * `app_response_time` - For more advanced use cases in which an in-container monitor exists to monitor app response time
/// * `network_latency` - For more advanced use cases in which an in-container monitor exists to monitor external response time
/// * `disk_bandwidth` - Generally applicable disk bandwidth usage for any container (Gathered from the runtime if supported)
///
/// # Example
/// ```
/// let metric = InstanceMetrics {
///    cpu_load: Some(50),
///    ram_pressure: Some(50),
///    ram_usage: Some(50),
///    clients: Some(50),
///    app_response_time: Some(50),
///    network_latency: Some(50),
///    disk_bandwidth: Some(50),
/// };
/// ```
#[derive(Default, Debug)]
struct InstanceMetrics {
    cpu_load:          Option<u64>,
    ram_pressure:      Option<u64>,
    ram_usage:         Option<u64>,
    clients:           Option<u64>,
    app_response_time: Option<u64>,
    network_latency:   Option<u64>,
    disk_bandwidth:    Option<u64>,
}

impl InstanceMetrics {
    fn new(
        cpu_load:          Option<u64>,
        ram_pressure:      Option<u64>,
        ram_usage:         Option<u64>,
        clients:           Option<u64>,
        app_response_time: Option<u64>,
        network_latency:   Option<u64>,
        disk_bandwidth:    Option<u64>
    ) -> Self {
        InstanceMetrics {
            cpu_load,
            ram_pressure,
            ram_usage,
            clients,
            app_response_time,
            network_latency,
            disk_bandwidth,
        }
    }
    fn get_fields(&self) -> Vec<(&str, Option<u64>)> {
        vec![
            ("cpu_load", self.cpu_load),
            ("ram_pressure", self.ram_pressure),
            ("ram_usage", self.ram_usage),
            ("clients", self.clients),
            ("app_response_time", self.app_response_time),
            ("network_latency", self.network_latency),
            ("disk_bandwith", self.disk_bandwidth)
        ]
    }
    fn set_field(&mut self, field_name: &str, field_value: Option<u64>) {}
}

enum ScaleAction {
    ScaleUp,
    ScaleDown,
    ScaleLeft,
    ScaleRight,
    NoAction,
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum ResourceType {
    CPU,
    RAM,
    Clients,
    ResponseTime,
}

struct AutoscalerThresholds {
    cpu_threshold:           u64,
    ram_threshold:           u64,
    client_threshold:        u64,
    response_time_threshold: u64,
}

impl AutoscalerThresholds {
    fn new(
        cpu_threshold:           u64,
        ram_threshold:           u64,
        client_threshold:        u64,
        response_time_threshold: u64,
    ) -> Self {
        AutoscalerThresholds {
            cpu_threshold,
            ram_threshold,
            client_threshold,
            response_time_threshold,
        }
    }

    fn decide_all(&self, metrics: &InstanceMetrics) -> HashMap<ResourceType, ScaleAction> {
        let mut actions = HashMap::new();

        let cpu_action = match metrics.cpu_load {
            Some(cpu_load) if cpu_load > self.cpu_threshold => ScaleAction::ScaleUp,
            Some(cpu_load) if cpu_load < self.cpu_threshold / 2 => ScaleAction::ScaleDown,
            _ => ScaleAction::NoAction,
        };
        actions.insert(ResourceType::CPU, cpu_action);

        let ram_action = match metrics.ram_usage {
            Some(ram_usage) if ram_usage > self.ram_threshold => ScaleAction::ScaleUp,
            Some(ram_usage) if ram_usage < self.ram_threshold / 2 => ScaleAction::ScaleLeft,
            _ => ScaleAction::NoAction,
        };
        actions.insert(ResourceType::RAM, ram_action);

        let client_action = match metrics.clients {
            Some(clients) if clients > self.client_threshold => ScaleAction::ScaleRight,
            _ => ScaleAction::NoAction,
        };
        actions.insert(ResourceType::Clients, client_action);

        let response_time_action = match metrics.app_response_time {
            Some(response_time) if response_time > self.response_time_threshold => ScaleAction::ScaleRight,
            _ => ScaleAction::NoAction,
        };
        actions.insert(ResourceType::ResponseTime, response_time_action);

        actions // Return the actions to complete for each scaling category
    }
}

/// A struct that represents an AppInstance
/// # Fields
/// allocated_memory - Allocated memory to the app instance in MB
/// allocated_cpu - Allocated cpu to the app instance 100% per core
/// allocated_disk_bandwidth - Allocated disk to the app instance in MB/s
/// allocated_network_bandwidth - Allocated network to the app instance in MB/s
///
/// # Example
/// ```
/// let app_instance = AppInstance {
///   allocated_memory: 1024,
///   allocated_cpu: 100,
///   allocated_disk_bandwidth: 100,
///   allocated_network_bandwidth: 100,
/// };
/// ```
struct AppInstance {
    state: ApplicationState,
    allocated_memory:            u64,
    allocated_cpu:               u64,
    allocated_disk_bandwidth:    u64,
    allocated_network_bandwidth: u64,
}

/// An enum that represents the state of an application
/// # Variants
/// Healthy - Host is online and functioning within expected parameters.
/// Suspicious - Host is online but is exhibiting behavior that is outside of the expected parameters.
/// Erroneous - Host is exhibiting unexpected behavior and has been lowered on the load balancers priority list, avoiding routing traffic to the erroneous host if possible.
/// Blacklisted - Host has been blacklisted from the network by an operator.
/// Down - Host is offline or otherwise unreachable. (Its probably DNS's fault)
/// 
/// # Example
/// ```
/// let state = ApplicationState::Healthy;
/// ```
enum ApplicationState {
    Healthy,
    Suspicious,
    Erroneous,
    Blacklisted,
    Down
}

/// A trait that represents an AutoScaler
/// # Methods
/// scale - Scales the app instance based on the metrics
/// query - Queries the metrics
///
/// # Example
/// ```
/// struct ExampleScaler {
///    metrics: Arc<Mutex<Metri>
/// }
///
/// impl AutoScaler for ExampleScaler {
///   fn query(&self) -> InstanceMetric {
///    println!("Foo data");
///   InstanceMetric::default()
///  }
/// }
///
/// fn run() {
///  let example_scaler = ExampleScaler;
/// test(example_scaler);
/// }

enum ScaleResult {
    Success,
}

/// A trait that represents an AutoScaler
/// # Methods
/// scale - Scales the app instance based on the metrics
/// query - Queries the metrics
/// query_over_period - Queries the metrics over a period of time
/// reallocate_memory - Set new targets for the total memory avail to an app, this will be split among the instances which are rounded up
/// reallocate_cpu - Set new targets for the total cpu avail to an app, this will be split among the instances which are rounded up
/// reallocate_disk_bandwidth - Set new targets for the total disk avail to an app, this will be split among the instances which are rounded up
/// reallocate_network_bandwidth - Set new targets for the total network avail to an app, this will be split among the instances which are rounded up
/// 
/// # Example
/// ```
/// struct ExampleScaler {
///   metrics: Arc<Mutex<InstanceMetrics>>,
/// }
trait AutoScaler {
    fn scale(&mut self, instance: &mut AppInstance,action: ScaleAction) 
      -> ScaleResult;
    fn query(&self) -> InstanceMetrics;
    async fn query_over_period(&self, duration: Duration)
      -> Vec<(InstanceMetrics, chrono::DateTime<chrono::Utc>)>;
    fn reallocate_memory(&self, megabytes: u64);                      
    fn reallocate_cpu(&self, percentage: u64);                        
    fn reallocate_disk_bandwidth(&self, megabytes_per_second: u64);   
    fn reallocate_network_bandwidth(&self, megabytes_per_second: u64);
}

struct ExampleScaler {
    metrics: Arc<Mutex<InstanceMetrics>>,
}

impl AutoScaler for ExampleScaler {
    fn query(&self) -> InstanceMetrics {
        let mut metric_collection = InstanceMetrics::new(None, None, None, None, None, None, None);
        for field in self.metrics.try_lock().unwrap().get_fields() {
            println!("{field:#?}");
            metric_collection.set_field(field.0, field.1);
        }
        println!("{:#?}", self.metrics.lock().unwrap().get_fields());

        metric_collection
    }
    
    fn scale(&mut self, instance: &mut AppInstance, action: ScaleAction) -> ScaleResult {
        match action {
            ScaleAction::ScaleUp => {
                instance.allocated_cpu += 10;
                instance.allocated_memory += 1024;
            }
            ScaleAction::ScaleDown => {
                instance.allocated_cpu = instance.allocated_cpu.saturating_sub(10);
                instance.allocated_memory = instance.allocated_memory.saturating_sub(1024);
            }
            ScaleAction::ScaleLeft => {
                instance.allocated_disk_bandwidth = instance.allocated_disk_bandwidth.saturating_sub(10);
            }
            ScaleAction::ScaleRight => {
                instance.allocated_network_bandwidth += 10;
            }
            ScaleAction::NoAction => {}
        }
        ScaleResult::Success
    }
    
    async fn query_over_period(&self, duration: Duration) -> Vec<(InstanceMetrics, chrono::DateTime<chrono::Utc>)> {
        let mut results = Vec::new();
        let start = chrono::offset::Utc::now();
        let end = start + chrono::Duration::from_std(duration).unwrap();
        let mut current_time = start;

        while current_time < end {
            let metrics = self.query();
            results.push((metrics, current_time));
            current_time = chrono::offset::Utc::now();
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        results
    }
    
    fn reallocate_memory(&self, megabytes: u64) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.ram_usage = Some(megabytes);
        } else {
            eprintln!("Failed to acquire lock for reallocate_memory");
        }
    }
    
    fn reallocate_cpu(&self, percentage: u64) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.cpu_load = Some(percentage);
        } else {
            eprintln!("Failed to acquire lock for reallocate_cpu");
        }
    }
    
    fn reallocate_disk_bandwidth(&self, megabytes_per_second: u64) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.disk_bandwidth = Some(megabytes_per_second);
        } else {
            eprintln!("Failed to acquire lock for reallocate_disk_bandwidth");
        }
    }
    
    fn reallocate_network_bandwidth(&self, megabytes_per_second: u64) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.network_latency = Some(megabytes_per_second);
        } else {
            eprintln!("Failed to acquire lock for reallocate_network_bandwidth");
        }
    }
}

/// A function that runs the example
#[test]
fn run() {
    let metrics = InstanceMetrics::new(Some(75), Some(60), Some(100), Some(150), Some(200), Some(50), Some(100));
    let autoscaler = AutoscalerThresholds::new(80, 75, 100, 180);

    let actions = autoscaler.decide_all(&metrics);
    for (resource, action) in actions {
        match action {
            ScaleAction::ScaleUp => println!("Scaling up {:?}...", resource),
            ScaleAction::ScaleDown => println!("Scaling down {:?}...", resource),
            ScaleAction::ScaleLeft => println!("Scaling left {:?}...", resource),
            ScaleAction::ScaleRight => println!("Scaling right {:?}...", resource),
            ScaleAction::NoAction => println!("No scaling action needed for {:?}.", resource),
        }
    }
}

//#[cfg(test)]
fn test(scaler: impl AutoScaler) {
    scaler.query();
}
