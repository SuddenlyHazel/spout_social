#![allow(unused)]

use std::time::Duration;

use futures::StreamExt;
use iroh::{NodeAddr, NodeId};
use iroh_docs::{
    rpc::{
        client::docs::ShareMode::{self, Read},
        AddrInfoOptions,
    },
    store::Query,
    DocTicket, NamespaceId,
};
use rinf::debug_print;
use sled::Db;

use crate::{
    messages::{
        owner_post_action::Action::{Delete, Update},
        CreatePostRequest, LogPostsTicket, OwnerPostAction, Post, PostQueryResponse,
        PostsRequestQuery,
    },
    node::protocol::client::{OceanProtocolClient, OceanProtocolClientBuilder},
    BlobsClient, DocsClient, SpoutDoc,
};
use chrono::Utc;

static POSTS: &'static str = "USER_POSTS";

#[derive(Clone)]
pub struct PostsHandle {
    user_posts: SpoutDoc,
}

impl PostsHandle {
    pub async fn user_posts_ticket(&self) -> anyhow::Result<DocTicket> {
        Ok(self
            .user_posts
            .share(ShareMode::Read, AddrInfoOptions::Relay)
            .await?)
    }

    pub async fn list_peers(&self) -> anyhow::Result<Vec<NodeId>> {
        let Some(peers) = self.user_posts.get_sync_peers().await? else {
            return Ok(vec![]);
        };
        Ok(peers
            .iter()
            .map(|v| NodeId::from_bytes(v).unwrap())
            .collect::<Vec<_>>())
    }
}

async fn load_user_posts_doc(app_db: &Db, docs: &DocsClient) -> anyhow::Result<SpoutDoc> {
    let doc = match app_db.get(POSTS)? {
        Some(key) => {
            let namespace_id: NamespaceId =
                serde_json::from_slice(&key).expect("Failed to deserialize posts namespaceId");
            docs.open(namespace_id)
                .await
                .expect("Failed to open Posts Doc")
                .expect("Post document wasnt found")
        }
        None => {
            let doc = docs.create().await?;
            app_db
                .insert(
                    POSTS,
                    serde_json::to_vec(&doc.id()).expect("Failed to serialize documentId"),
                )
                .expect("Failed to write posts id to app_db");
            debug_print!("store posts key flush {:?}", app_db.flush());
            doc
        }
    };
    Ok(doc)
}

pub async fn start_actors(
    ocean_client: OceanProtocolClient,
    app_db: Db,
    docs: DocsClient,
    blobs: BlobsClient,
    author_id: iroh_docs::AuthorId,
) -> anyhow::Result<PostsHandle> {
    let doc = load_user_posts_doc(&app_db, &docs).await?;
    tokio::task::spawn(posts_create_actor(app_db.clone(), doc.clone(), author_id));
    tokio::task::spawn(posts_query_actor(
        app_db.clone(),
        doc.clone(),
        blobs,
        author_id,
        ocean_client.clone(),
    ));
    tokio::task::spawn(post_actions_actor(app_db.clone(), doc.clone(), author_id));
    Ok(PostsHandle { user_posts: doc })
}

async fn posts_query_actor(
    app_db: Db,
    doc: SpoutDoc,
    blobs: BlobsClient,
    author_id: iroh_docs::AuthorId,
    ocean_client: OceanProtocolClient,
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

        if let Ok(mut ocean_posts) = ocean_client.request_ocean_posts().await {
            debug_print!("Ayee we got posts! {}", ocean_posts.len());
            posts.extend(ocean_posts.into_iter());
        }

        PostQueryResponse { posts }.send_signal_to_dart();
    }
    Ok(())
}

async fn post_actions_actor(
    app_db: Db,
    doc: SpoutDoc,
    author_id: iroh_docs::AuthorId,
) -> anyhow::Result<()> {
    let recv = OwnerPostAction::get_dart_signal_receiver();

    while let Some(action) = recv.recv().await {
        let post_id = &action.message.post_id;
        match action.message.action() {
            Delete => {
                println!("Trying to delete post");
                let res = doc.del(author_id.clone(), post_id.clone()).await;
                println!("delete result.. {res:?}");
            }
            Update => todo!(),
        }
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
        let recv = LogPostsTicket::get_dart_signal_receiver();

        while let Some(_) = recv.recv().await {
            let ticket = doc
                .share(Read, AddrInfoOptions::Id)
                .await
                .expect("Failed to create debug share ticket");
            debug_print!("{}", ticket.to_string());
            println!("{:#?}", ticket);
        }
    });

    let _doc = doc.clone();
    tokio::task::spawn(async move {
        let doc = _doc.clone();
        let mut timer = tokio::time::interval(Duration::from_secs(30));

        let mut peers = String::new();
        loop {
            timer.tick().await;
            let new_peers_maybe = format!("{:#?}", doc.get_sync_peers().await);
            if peers != new_peers_maybe {
                peers = new_peers_maybe;
                println!("Peers updated {}", peers);
            }
        }
    });

    while let Some(req) = requests.recv().await {
        println!("Received post request");
        let created_at = Utc::now().timestamp_millis();
        let CreatePostRequest { title, body } = req.message;
        let author = "@place_holder".into();

        let key = format!("post.{}", created_at);

        let post = {
            let author_id = author_id.clone().to_string();
            Post {
                post_id: key.clone(),
                title,
                body,
                created_at,
                author,
                author_id,
            }
        };

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
