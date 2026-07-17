use leptos::prelude::*;
use std::sync::{LazyLock, Mutex};
use serde::{Deserialize, Serialize};
// use rand::RngExt;
// use serde_json;
// pub struct Post {
//     post: PostContent,
//     comments: Vec<Comment>
// }


#[derive(Deserialize, Serialize, Clone)]
pub struct PostContent {
    pub user: String,
    pub post_data: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Comment {
    pub user: String,
    pub comment: String
}

#[component]
pub fn BlogPost(post : PostContent) -> impl IntoView {
    view!{
        <div class="post">
            <p>{post.user}</p>
            <p>{post.post_data}</p>
        </div>
    }
}


#[component]
pub fn BlogPostComment(comment : Comment) -> impl IntoView {
    view!{
        <div class="comment">
            <p>{comment.user}</p>
            <p>{comment.comment}</p>
        </div>   
    }
}

// i only need the resource content ???
    // let post_data = Resource::new_blocking(/* load blog post */);
    // let comments_data = Resource::new(/* load blog comments */);
// deez...

static BLOGPOSTS : LazyLock<Mutex<Vec<PostContent>>> = LazyLock::new(|| Mutex::new({
    let mut v = Vec::new();
    v.push(PostContent{
        user: "A".to_string(),
        post_data: "Post 1".to_string()
    });
    v.push(PostContent{
        user: "B".to_string(),
        post_data: "Post 2".to_string()
    });
    v.push(PostContent{
        user: "C".to_string(),
        post_data: "Post 3".to_string()
    });
    v
}));

static COMMENTS : LazyLock<Mutex<Vec<Comment>>> = LazyLock::new(|| Mutex::new({
    let mut v = Vec::new();
    v.push(Comment{
        user: "A".to_string(),
        comment: "Comment 1".to_string()
    });
    v.push(Comment{
        user: "B".to_string(),
        comment: "Comment 2".to_string()
    });
    v.push(Comment{
        user: "C".to_string(),
        comment: "Comment 3".to_string()
    });
    v
}));

// server fn? [something??]
// #[server]
pub async fn get_posts() -> Result<Vec<PostContent>, ServerFnError> {
    // let mut rng = rand::rng();
    // let w : u32 = rng.random_range(1..4);
    // wait_for(w).await;
    // {
    //     TimeoutFuture::new(2_000).await;
    // }
    // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok(BLOGPOSTS.lock().unwrap().clone())
}

#[server]
pub async fn get_post(index: usize) -> Result<PostContent, ServerFnError> {
    // add random/optional delay?
    Ok(BLOGPOSTS.lock().unwrap()[index].clone())
}

// #[server]
pub async fn get_comments() -> Result<Vec<Comment>, ServerFnError> {
    Ok(COMMENTS.lock().unwrap().clone())
}

#[server]
pub async fn get_comment(index: usize) -> Result<Comment, ServerFnError> {
    // add random/optional delay?
    Ok(COMMENTS.lock().unwrap()[index].clone())
}