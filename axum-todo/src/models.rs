use anyhow::Result;
use libsql_client::client::Client;

use libsql_client::Statement;
use serde::Serialize;

use uuid::Uuid;

pub type Db = dashmap::DashMap<Uuid, Todo>;

#[derive(Debug, Serialize, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub text: String,
    pub completed: bool,
}

pub struct SqliteDb {
    client: Client,
}

impl SqliteDb {
    /// Creates a new database client.
    /// If the LIBSQL_CLIENT_URL environment variable is not set, a local database will be used.
    /// It's also possible to use a remote database by setting the LIBSQL_CLIENT_URL environment variable.
    /// The `mail` table will be automatically created if it does not exist.
    pub async fn new() -> Result<Self> {
        if std::env::var("LIBSQL_CLIENT_URL").is_err() {
            let mut db_path = std::env::temp_dir();
            db_path.push("todos.db");
            let db_path = db_path.display();
            tracing::warn!("LIBSQL_CLIENT_URL not set, using a default local database: {db_path}");
            std::env::set_var("LIBSQL_CLIENT_URL", format!("file://{db_path}"));
        }
        let db = Client::from_env().await?;
        db.batch(["CREATE TABLE IF NOT EXISTS todo (id text, text text, completed completed)"])
            .await?;
        Ok(Self { client: db })
    }
    pub async fn get_todos(&self) -> Result<Vec<Todo>> {
        let mut todos = Vec::new();
        let rs = self.client.execute("SELECT * FROM todo").await?;
        for row in &rs.rows {
            let id: &str = row.try_get(0)?;
            let text: &str = row.try_get(1)?;
            let completed: usize = row.try_get(2)?;

            todos.push(Todo {
                id: Uuid::parse_str(&id)?,
                text: text.to_string(),
                completed: completed == 1,
            });
        }
        Ok(todos)
    }
    pub async fn save_todo(&self, id: Uuid, text: String, completed: bool) -> Result<()> {
        self.client
            .execute(Statement::with_args(
                "INSERT INTO todo VALUES(?,?,?)",
                libsql_client::args!(id.to_string(), text, completed as usize),
            ))
            .await?;
        Ok(())
    }
}
