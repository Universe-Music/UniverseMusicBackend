use log::{error, info};
use rbatis::{intercept::Intercept, plugin::intercept_log::LogInterceptor, RBatis};
use rbdc_mysql::driver::MysqlDriver;
use std::sync::{Arc, OnceLock};

pub struct DBPool {
    conn: RBatis,
}

impl DBPool {
    pub async fn init_connect(url: &str) -> DBPool {
        let rb = RBatis::new();
        let intercepts = Arc::new(LogInterceptor::new(log::LevelFilter::Debug));
        rb.intercepts.clear();
        rb.intercepts.push(intercepts.clone() as Arc<dyn Intercept>);

        if let Err(e) = rb.link(MysqlDriver {}, url).await {
            error!("failed to connect to database, continue anyway: {:?}", e);
        };
        let ver: String = rb
            .query_decode("select version();", vec![])
            .await
            .unwrap_or(String::from("Unknown ver."));
        info!("database version: {}", ver);
        DBPool { conn: rb }
    }
}
