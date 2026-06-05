use sqlx::{ MySql, MySqlPool, Transaction };

pub struct MySqlUnitOfWork<'a> {
    tx: Option<Transaction<'a, MySql>>
}

impl<'a> MySqlUnitOfWork<'a> {
    pub async fn begin(pool: &'a MySqlPool) -> Result<Self, String> {
        let tx = pool.begin().await.map_err(|e| e.to_string())?;
        
        Ok(Self { tx: Some(tx) })
    }

    pub async fn commit(mut self) -> Result<(), String> {
        if let Some(tx) = self.tx.take() {
            tx.commit().await.map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    pub async fn rollback(mut self) -> Result<(), String> {
        if let Some(tx) = self.tx.take() {
            tx.rollback().await.map_err(|e| e.to_string())?;
        }

        Ok(())
    }
    
    pub fn tx(&mut self) -> &mut Transaction<'a, MySql> {
        self.tx.as_mut().unwrap()
    }
}
