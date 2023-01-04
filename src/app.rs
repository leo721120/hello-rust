#[derive(Clone, Debug)]
pub struct App {
    pub pool: sqlx::AnyPool,
}

/*pub struct RFC7807 {
    pub title: String,
}*/