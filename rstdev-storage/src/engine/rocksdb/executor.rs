use rst_common::with_tokio::tokio::task::spawn_blocking;

use super::db::DB;
use super::types::{Instruction, OutputOpts, RocksDBError};

#[derive(Clone)]
pub struct Executor {
    instance: DB,
    cf_name: String,
}

impl Executor {
    pub fn new(instance: DB, cf_name: String) -> Self {
        Self { instance, cf_name }
    }

    pub async fn exec(&self, instruction: Instruction) -> Result<OutputOpts, RocksDBError> {
        let db_instance =
            self.instance
                .db
                .clone()
                .map(|val| val)
                .ok_or(RocksDBError::ExecutorError(
                    "missing database instance".to_string(),
                ))?;

        let cf_name = self.cf_name.clone();

        match instruction {
            Instruction::SaveCf { key, value } => {
                let _ = spawn_blocking(move || {
                    let cf = db_instance
                        .cf_handle(cf_name.as_str())
                        .map(|val| val.to_owned())
                        .ok_or(RocksDBError::ExecutorError("cf handler failed".to_string()))?;

                    let result = db_instance
                        .put_cf(cf, key, value)
                        .map_err(|err| RocksDBError::ExecutorError(err.to_string()));

                    result
                })
                .await
                .map_err(|err| RocksDBError::ExecutorError(err.to_string()))??;

                Ok(OutputOpts::None)
            }
            Instruction::MergeCf { key, value } => {
                let _ = spawn_blocking(move || {
                    let cf = db_instance
                        .cf_handle(cf_name.as_str())
                        .map(|val| val.to_owned())
                        .ok_or(RocksDBError::ExecutorError("cf handler failed".to_string()))?;

                    let result = db_instance
                        .merge_cf(cf, key, value)
                        .map_err(|err| RocksDBError::ExecutorError(err.to_string()));

                    result
                })
                .await
                .map_err(|err| RocksDBError::ExecutorError(err.to_string()))??;

                Ok(OutputOpts::None)
            }
            Instruction::GetCf { key } => {
                let value = spawn_blocking(move || {
                    let cf = db_instance
                        .cf_handle(cf_name.as_str())
                        .map(|val| val.to_owned())
                        .ok_or(RocksDBError::ExecutorError("cf handler failed".to_string()))?;

                    let result = db_instance
                        .get_cf(cf, key)
                        .map_err(|err| RocksDBError::ExecutorError(err.to_string()))
                        .map(|val| val);

                    result
                })
                .await
                .map_err(|err| RocksDBError::ExecutorError(err.to_string()))??;

                Ok(OutputOpts::SingleByte { value })
            }
            Instruction::MultiGetCf { keys } => {
                let value = spawn_blocking(move || {
                    let cf = db_instance
                        .cf_handle(cf_name.as_str())
                        .map(|val| val.to_owned())
                        .ok_or(RocksDBError::ExecutorError("cf handler failed".to_string()))?;

                    let cf_keys = keys.iter().map(|val| (cf, val));
                    let result = db_instance.multi_get_cf(cf_keys);

                    Ok(result)
                })
                .await
                .map_err(|err| RocksDBError::ExecutorError(err.to_string()))??
                .iter()
                .map(|val| match val.to_owned() {
                    Ok(ok_val) => Ok(ok_val),
                    Err(err) => Err(RocksDBError::ExecutorError(err.to_string())),
                })
                .collect();

                Ok(OutputOpts::MultiBytes { values: value })
            }
            Instruction::RemoveCf { key } => {
                let _ = spawn_blocking(move || {
                    let cf = db_instance
                        .cf_handle(cf_name.as_str())
                        .map(|val| val.to_owned())
                        .ok_or(RocksDBError::ExecutorError("cf handler failed".to_string()))?;

                    let result = db_instance
                        .delete_cf(cf, key)
                        .map_err(|err| RocksDBError::ExecutorError(err.to_string()))
                        .map(|val| val);

                    result
                })
                .await
                .map_err(|err| RocksDBError::ExecutorError(err.to_string()))??;

                Ok(OutputOpts::None)
            }
        }
    }
}
