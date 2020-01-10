use std::fmt::Display;

use crate::arguments::Arguments;
use crate::connection::Connection;
use crate::row::Row;

/// A database driver.
///
/// This trait encapsulates a complete driver implementation to a specific
/// database (e.g., MySQL, Postgres).
pub trait Database: 'static {
    /// The concrete `Connection` implementation for this database.
    type Connection: Connection<Database = Self>;

    /// The concrete `Arguments` implementation for this database.
    type Arguments: Arguments<Database = Self>;

    /// The concrete `Row` implementation for this database.
    type Row: Row<Database = Self>;

    /// The Rust type of type identifiers for this database.
    type TypeId: Display + Copy + PartialEq<Self::TypeId>;

    /// The Rust type of table identifiers for this database.
    type TableId: Display;
}
