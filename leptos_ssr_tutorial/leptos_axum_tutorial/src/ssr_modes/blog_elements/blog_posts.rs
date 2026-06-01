use leptos::prelude::*;

pub struct Post {
    post: PostContent,
    comments: Vec<Comment>
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