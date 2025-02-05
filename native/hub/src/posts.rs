use std::time::Duration;

use anyhow::Context;
use futures::StreamExt;
use iroh::{NodeAddr, NodeId};
use iroh_blobs::get::request;
use iroh_docs::{
    rpc::client::docs::ShareMode::{self, Read},
    store::Query,
    NamespaceId,
};
use sled::Db;

use crate::{
    messages::{CreatePostRequest, LogPostsTicket, Post, PostQueryResponse, PostsRequestQuery},
    BlobsClient, DocsClient, SpoutDoc,
};
use chrono::Utc;

static POSTS: &'static str = "USER_POSTS";

pub async fn start_actors(
    app_db: Db,
    docs: DocsClient,
    blobs: BlobsClient,
    author_id: iroh_docs::AuthorId,
) {
    let doc = if let Ok(Some(key)) = app_db.get(POSTS) {
        let namespace_id: NamespaceId =
            serde_json::from_slice(&key).expect("Failed to deserialize posts namespaceId");
        docs.open(namespace_id)
            .await
            .expect("Failed to open Posts Doc")
            .expect("Post document wasnt found")
    } else {
        let doc = docs
            .create()
            .await
            .expect("Failed to create posts document");
        app_db
            .insert(
                POSTS,
                serde_json::to_vec(&doc.id()).expect("Failed to serialize documentId"),
            )
            .expect("Failed to write posts id to app_db");
        doc
    };
    tokio::task::spawn(posts_create_actor(app_db.clone(), doc.clone(), author_id));
    tokio::task::spawn(posts_query_actor(app_db, doc, blobs, author_id));
}
async fn posts_query_actor(
    app_db: Db,
    doc: SpoutDoc,
    blobs: BlobsClient,
    author_id: iroh_docs::AuthorId,
) -> anyhow::Result<()> {
    let requests = PostsRequestQuery::get_dart_signal_receiver();

    while let Some(request) = requests.recv().await {
        let PostsRequestQuery { start_at, amount } = request.message;
        let q = Query::key_prefix("post.")
            .sort_by(
                iroh_docs::store::SortBy::KeyAuthor,
                iroh_docs::store::SortDirection::Desc,
            )
            .limit(amount as u64);

        let mut docs = doc.get_many(q).await?;

        let mut posts = vec![];
        while let Some(Ok(entry)) = docs.next().await {
            let post_author = entry.author();
            let post_content_hash = entry.content_hash();
            let post_content = blobs.read(post_content_hash).await?.read_to_bytes().await?;
            match serde_json::from_slice(&post_content) {
                Ok(post) => posts.push(post),
                Err(_) => {
                    println!("Malformed post removing for now");
                    let id = entry.id();
                    let id = id.clone();
                    let id = id.key().to_vec();
                    doc.del(author_id, id).await;
                }
            };
        }

        PostQueryResponse { posts }.send_signal_to_dart();
    }
    Ok(())
}

async fn posts_create_actor(
    app_db: Db,
    doc: SpoutDoc,
    author_id: iroh_docs::AuthorId,
) -> anyhow::Result<()> {
    let requests = CreatePostRequest::get_dart_signal_receiver();

    let _doc = doc.clone();

    tokio::task::spawn(async move {
        let doc = _doc;
        let mut recv = LogPostsTicket::get_dart_signal_receiver();

        while let Some(_) = recv.recv().await {
            let ticket = doc
                .share(Read, iroh_docs::rpc::AddrInfoOptions::RelayAndAddresses)
                .await
                .expect("Failed to create debug share ticket");
            println!("{}", ticket.to_string());
            println!("{:#?}", ticket);
        }
    });

    let _doc = doc.clone();
    tokio::task::spawn(async move {
        let doc = _doc.clone();
        let mut timer = tokio::time::interval(Duration::from_secs(30));
        loop {
            timer.tick().await;
            println!("{:#?}", doc.get_sync_peers().await);
        }
    });

    while let Some(req) = requests.recv().await {
        println!("Received post request");
        let created_at = Utc::now().timestamp_millis();
        let CreatePostRequest { title, body } = req.message;
        let author = "@place_holder".into();

        let post = {
            let author_id = author_id.clone().to_string();
            Post {
                title,
                body,
                created_at,
                author,
                author_id,
            }
        };

        let key = format!("post.{}", created_at);
        println!("Created post {post:?}");
        doc.set_bytes(author_id, key, serde_json::to_vec(&post)?)
            .await?;

        if let Ok(Some(peers)) = doc.get_sync_peers().await {
            let peers = peers
                .into_iter()
                .map(|v| {
                    NodeAddr::new(
                        NodeId::from_bytes(&v).expect("Failed to construct Node from Peer"),
                    )
                })
                .collect::<Vec<_>>();

            let res = doc.start_sync(peers).await;
            println!("SYNC RESULT {res:?}");
        }
    }
    Ok(())
}
