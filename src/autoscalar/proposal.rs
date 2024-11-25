use std::sync::{Arc, Mutex};

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
    fn get_fields(&self) -> Vec<(&str,Option<u64>)> {
        vec![
            ("cpu_load",self.cpu_load),
            ("ram_pressure",self.ram_pressure),
            ("ram_usage",self.ram_usage),
            ("clients",self.clients),
            ("app_response_time",self.app_response_time),
            ("network_latency",self.network_latency),
            ("disk_bandwith",self.disk_bandwidth),
        ]
    }
    fn set_field(&mut self,field_name: &str, field_value: Option<u64>) {
        
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
    allocated_memory:            u64,
    allocated_cpu:               u64,
    allocated_disk_bandwidth:    u64,
    allocated_network_bandwidth: u64,
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

trait AutoScaler {
    fn scale(&mut self, instance: &mut AppInstance) {}
    fn query(&self) -> InstanceMetrics {
        InstanceMetrics::default()
    }
    fn allocate_memory(megabytes: u64) {}
    
}

struct ExampleScaler {
    metrics: Arc<Mutex<InstanceMetrics>>
}


impl AutoScaler for ExampleScaler {
    fn query(&self) -> InstanceMetrics {
        let mut metric_collection = InstanceMetrics::new(None,None,None,None,None,None,None);
        for field in self.metrics.try_lock().unwrap().get_fields() {
            println!("{field:#?}");
            metric_collection.set_field(field.0, field.1);

        }
        println!("{:#?}", self.metrics.lock().unwrap().get_fields());

        metric_collection
    }
}

/// A function that runs the example
#[test]
fn run() {
    let example_scaler = ExampleScaler {
        metrics:  Arc::new(Mutex::new(InstanceMetrics{
            cpu_load:          Some(25u64),
            ram_pressure:      Some(25u64),
            ram_usage:         Some(25u64),
            clients:           Some(25u64),
            app_response_time: Some(25u64),
            network_latency:   Some(25u64),
            disk_bandwidth:    Some(25u64),
        }))
    };
    test(example_scaler);
}


//#[cfg(test)]
fn test(scaler: impl AutoScaler) {
    scaler.query();
}