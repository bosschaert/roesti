pub mod service_registry;
pub mod location;
pub mod tidal_service;
pub mod sunlight_service;

#[macro_export]
macro_rules! dp {
  ( $( $x:expr ),* ) => {
    {
      $(
        println!("x: {}", $x);
        println!("x: {}", $x);
      )*
    }
  }
}