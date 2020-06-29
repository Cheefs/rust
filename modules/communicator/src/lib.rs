// declaration a module
pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
  // use super::client; // can use super like this
    #[test]
    fn it_works() {
      super::client::connect();
    }
}