// This file is a placeholder to get an idea of how our json files are structured to be able to
// eventually graph them
//
// Some fields are optional, how to represent in SQL???

pub struct deezer_structure {
    obj
        link: String
        fans
            weekly_diff: i32
            weekly_diff_percent: f32
            monthly_diff: i32
            monthly_diff_percent: f32
            value: u32
            timestp: String
            daily_diff: i16
            interpolated: bool
}

pub struct instagram_structure {
    obj
        link: String
        followers
            weekly_diff: i32
            weekly_diff_percent: f32
            monthly_diff: i32
            monthly_diff_percent: f32
            value: u32
            timestp: String
            flags: bool
            daily_diff: i16
            interpolated: bool
}

pub struct soundcloud_structure {
    obj
        link: String
        followers 
            weekly_diff: i32
            weekly_diff_percent: f32
            monthly_diff: i32
            monthly_diff_percent: f32
            value: u32
            timestp: String
            daily_diff: i16
            interpolated: bool

}

pub struct spotify_structure {
    obj
        link: String
        followers
            weekly_diff: i32
            weekly_diff_percent: f32
            monthly_diff: i32
            monthly_diff_percent: f32
            value: u32
            timestp: String
            daily_diff: i16
            interpolated: bool
        // Popularity category seems very sparse, might be a waste of space
        popularity
             weekly_diff: i32
            weekly_diff_percent: f32
            monthly_diff: i32
            monthly_diff_percent: f32
            value: u32
            timestp: String
        // Highly skeptical of accuracy in Listeners category, might want to remove
        listeners
            weekly_diff: i32
            weekly_diff_percent: f32
            monthly_diff: i32
            monthly_diff_percent: f32
            value: u32
            timestp: String
            daily_diff: i16
            interpolated: bool
        followers_to_listeners_ratio
            weekly_diff: i32
            weekly_diff_percent: f32
            monthly_diff: i32
            monthly_diff_percent: f32
            value: u32
            timestp: String
            daily_diff: i16
            interpolated: bool
}

// probably remove entire struct
pub struct tiktok_structure {
    obj
        link
        followers
        likes
}

pub struct twitter_structure {
    obj
        link: String
        followers
            weekly_diff: i32
            weekly_diff_percent: f32
            monthly_diff: i32
            monthly_diff_percent: f32
            value: u32
            timestp: String
            flags: bool
            daily_diff: i16
            interpolated: bool
        // Retweets category seems to be empty, maybe remove
        retweets
            value: i16
            timestp: String
            flags: bool
            daily_diff: f32
            
}
// ********* probably remove entire struct
pub struct youtube_artist_structure {
    obj
        link: String
        // Highly skeptical of daily_views category accuracy
        daily_views
            weekly_diff: i32
            weekly_diff_percent: f32
            monthly_diff: i32
            monthly_diff_percent: f32
            value: u32
            timestp: String
            daily_diff: i16
            interpolated: bool
        // Highly skeptical of monthly_views category accuracy
        monthly_views
            weekly_diff: i32
            weekly_diff_percent: f32
            monthly_diff: i32
            monthly_diff_percent: f32
            value: u32
            timestp: String
            daily_diff: i16
            interpolated: bool
}

pub struct youtube_channel_structure {
    obj
        link: String
        // Highly skeptical
        subscribers
            weekly_diff: i32
            weekly_diff_percent: f32
            monthly_diff: i32
            monthly_diff_percent: f32
            value: u32
            timestp: String
            daily_diff: i16
            interpolated: bool
        views
            // skeptical of weekly_diff
            weekly_diff: i32
            weekly_diff_percent: f32
            monthly_diff: i32
            monthly_diff_percent: f32
            value: u32
            timestp: String
            daily_diff: i16
            interpolated: bool
        // null, remove category
        comments
        // useless, remove category
        videos
           value: i16
           timestp: String
           daily_diff: i16
           interpolated: bool
}
