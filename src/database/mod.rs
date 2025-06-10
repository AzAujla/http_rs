use rusqlite::Connection;

pub enum DatabaseConnectionType {
    Sqlite,
    Postgress,
    MySql,
}

pub fn connect(database_type: DatabaseConnectionType, url: String) -> Result<Connection, String> {
    return match database_type {
        DatabaseConnectionType::Sqlite => {
            let conn = Connection::open(url);
            return match conn {
                Ok(conn) => Ok(conn),
                Err(e) => Err(e.to_string()),
            };
        }
        _ => Err("Unimplemented Database Type".into()),
    };
}
