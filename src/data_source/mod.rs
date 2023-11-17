use lazy_static::lazy_static;
use once_cell::sync::OnceCell;

use crate::prelude::UserDataSource;

#[derive(Debug)]
pub struct DataSource {
    pub(crate) user: Box<dyn UserDataSource + Sync + Send>,
}

impl DataSource {
    pub fn new(user: Box<dyn UserDataSource + Sync + Send>) -> Self {
        Self { user }
    }

    pub fn use_as_default(self) {
        set_data_source(self)
    }
}

static mut DATA_SOURCE: OnceCell<DataSource> = OnceCell::new();

fn set_data_source(data_source: DataSource) {
    unsafe {
        DATA_SOURCE
            .set(data_source)
            .expect("Error set global data_source");
    }
}

pub fn get_data_source() -> &'static DataSource {
    unsafe {
        DATA_SOURCE
            .get()
            .expect("Should init data source before use it")
    }
}
