use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Ok,
    Degraded,
    Down,
}

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    /// API status
    status: Status,

    /// The version of the API.
    version: String,

    /// The number of seconds the API has been running since startup.
    uptime_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainersResponse {
    /// All containers returned by the API.
    items: Vec<ApiContainerSummary>,

    /// The number of containers in the API response.
    count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a minimal version of `ContainerSummary` from Bollard, designed to avoid sending unnecessary data in API responses.
pub struct ApiContainerSummary {
    /// The unique identifier of the Docker container.
    pub id: String,

    /// The name of the Docker container.
    pub name: String,

    /// The image used to create the container.
    pub image: String,

    /// The current state of the container (e.g., running, exited).
    pub state: String,

    /// The status description of the container (e.g., "Up 5 minutes").
    pub status: String,

    /// The creation timestamp of the container (Unix time).
    pub created: i64,
}
