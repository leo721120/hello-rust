pub mod controller;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FooAttributes {
    username: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FooIdentity {
    id: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Foo {
    id: String,
    username: Option<String>,
}

pub mod service {
    use crate::app::App;
    use crate::foo::FooAttributes;
    use crate::foo::FooIdentity;

    pub async fn create_foo(
        app: App,
        params: FooAttributes,
    ) -> anyhow::Result<FooIdentity> {
        let now = std::time::UNIX_EPOCH.elapsed()?.as_millis();
        let id = now.to_string();
        let row = sqlx::query(
            "
            INSERT INTO foo (id, name) VALUES
            ($1, $2)
            RETURNING id
            ",
        )
        .bind(id)
        .bind(params.username)
        .execute(&app.pool)
        .await?;

        tracing::info!("{:?}", row);

        //row.

        return Ok(FooIdentity { id: String::from("a") });
    }
}
