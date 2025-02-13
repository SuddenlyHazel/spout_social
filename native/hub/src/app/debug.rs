use std::time::Duration;

use iroh::{protocol::Router, Endpoint};
use rinf::debug_print;

use crate::{
    messages::{CurrentNodes, ListNodes},
    BlobsClient, DocsClient, SpoutBlobs, SpoutDocs,
};

use super::{posts::PostsHandle, profile::ProfilesHandle};

pub async fn current_nodes(
    posts_handle: PostsHandle,
    profiles_handle: ProfilesHandle,
) -> anyhow::Result<()> {
    let rx = ListNodes::get_dart_signal_receiver();
    loop {
        let Some(_) = rx.recv().await else { continue };
        let mut nodes = vec![];
        nodes.append(
            &mut posts_handle
                .list_peers()
                .await?
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
        );
        nodes.append(
            &mut profiles_handle
                .list_peers()
                .await?
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
        );
        debug_print!("{:?}", nodes);
        CurrentNodes { nodes }.send_signal_to_dart();
    }
}
