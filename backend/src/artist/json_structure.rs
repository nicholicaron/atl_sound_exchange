// Do we need rust_decimal instead of f64???
//
//
pub struct deezer_structure {
    obj
        link
        fans
            weekly_diff: i16
            weekly_diff_percent: f64
            monthly_diff: i32
            monthly_diff_percent: f64
            value: u32
            timestp: String
            daily_diff: i16
            interpolated: bool
}

pub struct instagram_structure {
    obj
        link
        followers

}

pub struct soundcloud_structure {
    obj
        link
        followers
}

pub struct spotify_structure {
    obj
        link
        followers
        popularity
        listeners
        followers_to_listeners_ratio
}

pub struct tiktok_structure {
    obj
        link
        followers
        likes
}

pub struct twitter_structure {
    obj
        link
        followers
        retweets
}

pub struct youtube_artist_structure {
    obj
        link
        daily_views
        monthly_views
}

pub struct youtube_channel_structure {
    obj
        link
        subscribers
        views
        comments
        videos
}
