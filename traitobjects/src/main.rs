use traitobjects::Post;

fn main() {
    let mut post = Post::new(); //draft state
    post.add_text("This is the first post here!!!");
    
    let post = post.request_review();
    
    let post = post.approve();
    assert_eq!("This is the first post here!!!", post.contents());
    
}
