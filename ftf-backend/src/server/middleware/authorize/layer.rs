use super::Authorize;
use tower_layer::Layer;
#[derive(Debug, Clone)]
pub enum AuthType {
    Basic,
    Vendor { vendor_id: String },
}

#[derive(Debug, Clone)]
pub struct AuthLayer {
    auth_type: AuthType,
}

impl AuthLayer {
    pub fn new(auth_type: AuthType) -> Self {
        AuthLayer { auth_type }
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = Authorize<S>;

    fn layer(&self, service: S) -> Self::Service {
        Authorize::new(service, self.auth_type.clone())
    }
}
