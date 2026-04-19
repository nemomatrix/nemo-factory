pub struct Lock;

impl Lock {
    pub fn enforce(dag_size: usize) -> Result<(), String> {
        if dag_size > 500 {
            return Err("DAG SIZE LIMIT EXCEEDED".into());
        }

        Ok(())
    }
}
