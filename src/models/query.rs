use rusqlite::Connection;

enum TableOrder {
    ASC,
    DESC,
}

enum TableAction {
    Get,
    Update,
    Delete,
    Create,
}

pub struct QueryBuilder<'a> {
    connection: &'a Connection,
    table: String,
    order_by: Option<(String, TableOrder)>,
    // bool will be false for OR and true for AND
    where_clause: Vec<(bool, String)>,
    table_action: Option<TableAction>,
}

impl<'a> QueryBuilder<'a> {
    pub fn new(connection: &'a Connection, table: &str) -> Self {
        QueryBuilder {
            connection,
            table: String::from(table),
            order_by: None,
            where_clause: Vec::new(),
            table_action: None,
        }
    }

    pub fn order_asc(mut self, column: &'a str) -> Self {
        self.order_by = Some((String::from(column), TableOrder::ASC));
        self
    }

    pub fn order_desc(mut self, column: &'a str) -> Self {
        self.order_by = Some((String::from(column), TableOrder::DESC));
        self
    }

    pub fn latest(mut self) -> Self {
        self.order_by = Some((String::from("created_at"), TableOrder::ASC));
        self
    }

    pub fn oldest(mut self) -> Self {
        self.order_by = Some((String::from("created_at"), TableOrder::ASC));
        self
    }

    pub fn and_where(mut self, clause: &'a str) -> Self {
        self.where_clause.push((true, String::from(clause)));
        self
    }

    pub fn or_where(mut self, clause: &'a str) -> Self {
        self.where_clause.push((false, String::from(clause)));
        self
    }

    pub fn get<T>(mut self) -> Vec<T> {
        self.table_action = Some(TableAction::Get);
        Vec::new()
    }

    pub fn create<T>(mut self, data: Vec<T>) -> bool {
        self.table_action = Some(TableAction::Create);
        true
    }

    pub fn delete(mut self) -> bool {
        self.table_action = Some(TableAction::Delete);
        true
    }
}
