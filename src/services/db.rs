use std::sync::OnceLock;
use tokio_postgres::types::{ToSql, Type};
use tokio_postgres::{Client, NoTls, Row, Statement};

static CLIENT: OnceLock<Client> = OnceLock::new();

pub struct DB;

impl DB {
    pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
        let user = std::env::var("POSTGRES_USER")?;
        let password = std::env::var("POSTGRES_PASSWORD")?;
        let conn_str = format!("host=postgres user={} password={}", user, password);

        let (client, conn) = tokio_postgres::connect(&conn_str, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("connection error: {}", e);
            }
        });

        let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await?;
        let value: &str = rows[0].get(0);
        assert_eq!(value, "hello world");

        CLIENT.set(client).unwrap();

        Ok(())
    }

    pub async fn prepare(
        query: &str,
        parameter_types: &[Type],
    ) -> Result<Statement, tokio_postgres::error::Error> {
        Self::get().prepare_typed(query, parameter_types).await
    }

    pub async fn query<T>(
        statement: &Statement,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<T>, tokio_postgres::error::Error>
    where
        T: From<Row>,
    {
        let rows = Self::get().query(statement, params).await?;
        Ok(rows.into_iter().map(|row| T::from(row)).collect())
    }

    pub async fn query_one<T>(
        statement: &Statement,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<T, tokio_postgres::error::Error>
    where
        T: From<Row>,
    {
        let row = Self::get().query_one(statement, params).await?;
        Ok(T::from(row))
    }

    pub async fn execute(
        statement: &Statement,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, tokio_postgres::error::Error> {
        Self::get().execute(statement, params).await
    }

    pub fn get() -> &'static Client {
        &CLIENT.get().unwrap()
    }
}
