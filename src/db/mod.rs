use crate::{config, utils};
use rbatis::rbatis::Rbatis;

lazy_static::lazy_static! {
    pub static ref RB:Rbatis = Rbatis::new();
}

pub async fn new_rb() {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        utils::get_env("MYSQL_USERNAME", config::DAFAULT_MYSQL_ROOT_USERNAME),
        utils::get_env("MYSQL_PASSWORD", config::DAFAULT_MYSQL_ROOT_PASSWORD),
        utils::get_env("MYSQL_SERVICE_HOST", config::DAFAULT_MYSQL_SERVICE_HOST),
        utils::get_env("MYSQL_SERVICE_PORT", config::DAFAULT_MYSQL_SERVICE_PORT),
        utils::get_env("MYSQL_DATABASE", config::DAFAULT_MYSQL_DB_NAME),
    );
    RB.link(&url).await.unwrap();
    RB.as_executor().exec(
        "create table if not exists envelope(envelope_id varchar(255) primary key not null,user_id varchar(255) not null,value int not null,opened boolean not null,snatch_time bigint not null);"
        , Vec::new()).await.unwrap();
}


#[cfg(test)]
mod test {
    use rbatis::crud::CRUD;
    use crate::utils;
    use crate::model;
    use super::*;
    #[tokio::test]
    pub async fn db_test() {
        fast_log::init_log("log/requests.log", 1000, log::Level::Info, None, true).unwrap();
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            utils::get_env("MYSQL_USERNAME", config::DAFAULT_MYSQL_ROOT_USERNAME),
            utils::get_env("MYSQL_PASSWORD", config::DAFAULT_MYSQL_ROOT_PASSWORD),
            utils::get_env("MYSQL_SERVICE_HOST", config::DAFAULT_MYSQL_SERVICE_HOST),
            utils::get_env("MYSQL_SERVICE_PORT", config::DAFAULT_MYSQL_SERVICE_PORT),
            utils::get_env("MYSQL_DATABASE", config::DAFAULT_MYSQL_DB_NAME),
        );
        RB.link(&url).await.unwrap();
        RB.as_executor().exec(
            "create table if not exists envelope(envelope_id varchar(255) primary key not null,user_id varchar(255) not null,value int not null,opened boolean not null,snatch_time bigint not null);"
            , Vec::new()).await.unwrap();
        let envelope = model::Envelope {
            envelope_id: String::from("_______this_nothing____rid_test1"),
            user_id: String::from("_______this_nothing____uid_test1"),
            opened: false,
            value: 0,
            snatch_time: 0,
        };
        RB.save(&envelope, &[]).await.unwrap();
        // model::add_envelope(&envelope).await.unwrap();
        let r: Option<model::Envelope> = RB.fetch_by_column("envelope_id", &envelope.envelope_id).await.unwrap();
        // println!("{:?}", r);
        let r = r.unwrap();
        assert_eq!(r, envelope);
        model::update_status_by_rid(&envelope.envelope_id).await.unwrap();
        assert_eq!(
            RB.remove_by_column::<model::Envelope, _>("envelope_id", &envelope.envelope_id).await.unwrap(),
            1
        );
    }
}

