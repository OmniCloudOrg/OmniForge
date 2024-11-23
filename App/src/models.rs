use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct ComponentStatus {
    #[tabled(rename = "Component")]
    pub name: String,
    #[tabled(rename = "Status")]
    pub status: String,
    #[tabled(rename = "Replicas")]
    pub replicas: String,
    #[tabled(rename = "CPU")]
    pub cpu: String,
    #[tabled(rename = "Memory")]
    pub memory: String,
}
