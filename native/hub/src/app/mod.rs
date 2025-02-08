use rinf::debug_print;

use crate::node::protocol::client::OceanProtocolClient;


pub async fn enter_ocean(ocean_client : OceanProtocolClient) -> anyhow::Result<()> {
  debug_print!("Entered the ocean task :D");
  Ok(())
}