use std::fmt::format;

use anyhow::Context;
use futures::StreamExt;
use iroh_blobs::get::request;
use iroh_docs::{store::Query, NamespaceId};
use sled::Db;

use crate::{
    messages::{CreatePostRequest, Post, PostQueryResponse, PostsRequestQuery},
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
    tokio::task::spawn(posts_query_actor(app_db, doc, blobs));
}
async fn posts_query_actor(app_db: Db, doc: SpoutDoc, blobs: BlobsClient) -> anyhow::Result<()> {
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
            let post: Post = serde_json::from_slice(&post_content)?;
            posts.push(post);
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
    while let Some(req) = requests.recv().await {
        println!("Received post request");
        let created_at = Utc::now().timestamp();
        let CreatePostRequest { title, body } = req.message;
        let post = Post {
            title,
            body,
            created_at,
        };

        let key = format!("post.{}", created_at);

        doc.set_bytes(author_id, key, serde_json::to_vec(&post)?)
            .await?;
    }
    Ok(())
}
