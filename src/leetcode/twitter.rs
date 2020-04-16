struct Twitter {
    //存放每个用户的关注
    followed:std::collections::HashMap<i32,std::collections::HashSet<i32>>,
    //存放所有推文
    news:std::collections::LinkedList<(i32,i32)>,
}//可以优化成每一个用户一个队列，订阅时merge队列


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self{
            followed:std::collections::HashMap::new(),
            news:std::collections::LinkedList::new(),
        }
    }
    
    /** Compose a new tweet. */
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.news.push_front((user_id,tweet_id));
    }
    
    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut res = vec![];
        if let Some(followed) = self.followed.get(&user_id) {
            for news in self.news.iter(){
                if (followed.contains(&news.0) || user_id == news.0) && res.len() < 10{
                    res.push(news.1);
                }
            }
            res
        }
        else{
            for news in self.news.iter(){
                if user_id == news.0 && res.len() < 10{
                    res.push(news.1);
                }
            }
            res
        }
    }
    
    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(followed) = self.followed.get_mut(&follower_id) {
            followed.insert(followee_id);
        }
        else{
            let mut s = std::collections::HashSet::new();
            s.insert(followee_id);
            self.followed.insert(follower_id,s);
        }
    }
    
    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(followed) = self.followed.get_mut(&follower_id) {
            followed.remove(&followee_id);
        }
    }
}
