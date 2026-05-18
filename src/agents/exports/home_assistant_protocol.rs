pub struct HomeAssistantSensorDiscovery {
    pub name: String,
    pub unique_id: String,
    pub state_topic: String,
    pub unit_of_measurement: String,
    pub device_class: String,
    pub state_class: String,
    pub device_identifier: String,
    pub device_name: String,
    pub manufacturer: String,
}

impl HomeAssistantSensorDiscovery {
    pub fn new(
        name: impl Into<String>,
        unique_id: impl Into<String>,
        state_topic: impl Into<String>,
        unit_of_measurement: impl Into<String>,
        device_class: impl Into<String>,
        state_class: impl Into<String>,
        device_identifier: impl Into<String>,
        device_name: impl Into<String>,
        manufacturer: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            unique_id: unique_id.into(),
            state_topic: state_topic.into(),
            unit_of_measurement: unit_of_measurement.into(),
            device_class: device_class.into(),
            state_class: state_class.into(),
            device_identifier: device_identifier.into(),
            device_name: device_name.into(),
            manufacturer: manufacturer.into(),
        }
    }

    pub fn to_json_string(&self) -> String {
        format!(
            "{{\"name\":\"{}\",\"unique_id\":\"{}\",\"state_topic\":\"{}\",\"unit_of_measurement\":\"{}\",\"device_class\":\"{}\",\"state_class\":\"{}\",\"device\":{{\"identifiers\":[\"{}\"],\"name\":\"{}\",\"manufacturer\":\"{}\"}}}}",
            self.name,
            self.unique_id,
            self.state_topic,
            self.unit_of_measurement,
            self.device_class,
            self.state_class,
            self.device_identifier,
            self.device_name,
            self.manufacturer,
        )
    }
}
