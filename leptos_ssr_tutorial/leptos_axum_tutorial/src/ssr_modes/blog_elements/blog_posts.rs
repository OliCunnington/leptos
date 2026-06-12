use leptos::prelude::*;
use gloo_timers::future::TimeoutFuture;
use std::sync::{LazyLock, Mutex};

// pub struct Post {
//     post: PostContent,
//     comments: Vec<Comment>
// }


// wait for rand... ?
pub async fn wait_for(i: i32) -> () {
    TimeoutFuture::new(i).await;
    ()
}

#[derive(Deserialize, Serialize)]
pub struct PostContent {
    user: String,
    postData: String
}

#[derive(Deserialize, Serialize)]
pub struct Comment {
    user: String,
    comment: String
}

#[component]
pub fn BlogPost(post : PostContent) -> impl IntoView {
    view!{
        <div class="post">
            <p>{post.user}</p>
            <p>{post.postData}</p>
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
    })
    v.push(PostContent{
        user: "B".to_string(),
        post_data: "Post 2".to_string()
    })
    v.push(PostContent{
        user: "C".to_string(),
        post_data: "Post 3".to_string()
    })
    v
}));

static COMMENTS : LazyLock<Mutex<Vec<Comment>>> = LazyLock::new(|| Mutex::new({
    let mut v = Vec::new();
    v.push(Comment{
        user: "A".to_string(),
        comment: "Comment 1".to_string()
    })
    v.push(Comment{
        user: "B".to_string(),
        comment: "Comment 2".to_string()
    })
    v.push(Comment{
        user: "C".to_string(),
        comment: "Comment 3".to_string()
    })
    v
}));

// server fn? [something??]

pub async fn get_posts() -> Result<Vec<PostContent>, ServerFnError> {
    // add random/optional delay?
    Ok(BLOGPOSTS.lock().unwrap().clone())
}

pub async fn get_post(index: i32) -> Result<PostContent, ServerFnError> {
    // add random/optional delay?
    Ok(BLOGPOSTS.lock().unwrap().clone()[index])
}

pub async fn get_comments() -> Result<Vec<Comment>, ServerFnError> {
    Ok(COMMENTS.lock().unwrap().clone())
}

pub async fn get_comment(index: i32) -> Result<Comment, ServerFnError> {
    // add random/optional delay?
    Ok(COMMENTS.lock().unwrap().clone()[index])
}