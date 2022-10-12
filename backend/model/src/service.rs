pub type ServiceId = String;

pub trait Service {
    fn info() -> &'static ServiceInfo;
}

#[derive(PartialEq, Debug, Clone)]
pub struct ServiceInfo {
    pub id: ServiceId,

    pub name: String,
    // TODO: some optional metadata
}

#[derive(PartialEq, Debug, Clone)]
pub struct ServiceReference(ServiceId);

#[derive(PartialEq, Debug, Clone)]
pub struct ServiceReferences(Vec<ServiceReference>);

#[cfg(test)]
mod test {
    use super::ServiceInfo;

    #[test]
    fn mock_service() {
        let _ = ServiceInfo {
            id: "builtin/auth".to_string(),
            name: "Authorization Service".to_string(),
        };
    }
}
