use std::any::{ Any };
use sqlx::{ MySql, Transaction };

use crate::domain::shared::{ TxContext };

pub struct MySqlTxContext {
    pub tx: Transaction<'static, MySql>
}

impl MySqlTxContext {
    pub fn new(tx: Transaction<'static, MySql>) -> Self {
        Self { tx }
    }
}

impl TxContext for MySqlTxContext {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        &mut *self
    }
}
