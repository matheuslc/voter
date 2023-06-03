struct Poll {
    poll_id: u64,
    poll_name: String,
}

struct Option {
    option_id: u64,
    option_name: String,
    option_order: u64,
    poll_id: u64,
}

struct Vote {
    vote_id: u64,
    option_id: u64,
    user_id: u64,
}