use crate::database::Database;

/// **Postgres** database driver.
pub struct Postgres;

impl Database for Postgres {
    type Connection = super::PgConnection;

    type Arguments = super::PgArguments;

    type Row = super::PgRow;

    type TypeId = super::PgTypeId;

    type TableId = u32;
}

impl_into_arguments_for_database!(Postgres);
