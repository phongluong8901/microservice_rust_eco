use dotenvy::var;

pub fn get_dsn() -> String {
    var("DSN").expect("Don't have DSN")
}

pub fn get_max_conns() -> u32 {
    let a = var("MAX_CONNS")
        .expect("Don't have MAX_CONNS")
        .parse::<u32>()
        .expect("Cann't parse max_conns");
    a
}

pub fn get_port() -> String {
    var("PORT").expect("Don't have PORT")
}
