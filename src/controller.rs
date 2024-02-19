mod anyma;
mod eurolite_pro;


pub trait Controller {
    fn send(&mut self, data: &[u8; 512]);
}

pub use anyma::AnymaController;
pub use eurolite_pro::EuroliteProController;
