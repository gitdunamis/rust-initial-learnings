use traitobjects::Post;

fn main() {
    let mut post: Post = Post::new(); //draft state
    
    post.add_text("This is the first post here!!!");
    assert_eq!("", post.contents());
    
    post.request_review();
    assert_eq!("", post.contents());
    
    post.approve();
    assert_eq!("This is the first post here!!!", post.contents());
    
}
