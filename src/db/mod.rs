use crate::utils;
use rbatis::rbatis::Rbatis;

lazy_static::lazy_static! {
    pub static ref RB:Rbatis = Rbatis::new();
}





pub async fn new_rb() {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        utils::get_env("MYSQL_ROOT_USERNAME", "root"),
        utils::get_env("MYSQL_ROOT_PASSWORD", "lvpiche"),
        utils::get_env("MYSQL_SERVICE_HOST", "localhost"),
        utils::get_env("MYSQL_SERVICE_PORT", "3306"),
        utils::get_env("MYSQL_DB_NAME", "MYDB"),
    );
    RB.link(&url).await.unwrap();
    RB.as_executor().exec(
        "create table if not exists envelope(rid varchar(255) primary key not null,uid varchar(255) not null,value int not null,status smallint not null,snatch_time bigint not null);"
        , Vec::new()).await.unwrap();
    // db max_connections
    // use crate::core::db::DBPoolOptions;
    // let mut opt =DBPoolOptions::new();
    // opt.max_connections=100;
    // rb.link_opt(url,&opt).await.unwrap();
    // log output
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true).unwrap();
}


#[cfg(test)]
mod test {
    use rbatis::crud::CRUD;
    use crate::utils;
    use crate::model;
    use super::*;
    #[tokio::test]
    pub async fn db_test() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true).unwrap();
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            utils::get_env("MYSQL_ROOT_USERNAME", "root"),
            utils::get_env("MYSQL_ROOT_PASSWORD", "lvpiche"),
            utils::get_env("MYSQL_SERVICE_HOST", "localhost"),
            utils::get_env("MYSQL_SERVICE_PORT", "3306"),
            utils::get_env("MYSQL_DB_NAME", "TEST"),
        );
        RB.link(&url).await.unwrap();
        RB.as_executor().exec(
            "create table if not exists envelope(rid varchar(255) primary key not null,uid varchar(255) not null,value int not null,status smallint not null,snatch_time bigint not null);"
            , Vec::new()).await.unwrap();
        let envelope = model::Envelope {
            rid: String::from("_______this_nothing____rid_test1"),
            uid: String::from("_______this_nothing____uid_test1"),
            status: 0,
            value: 0,
            snatch_time: 0,
        };
        RB.save(&envelope, &[]).await.unwrap();
        // model::add_envelope(&envelope).await.unwrap();
        let r= model::select_by_rid(&envelope.rid).await.unwrap();
        // println!("{:?}", r);
        assert_eq!(r, envelope);
        model::update_status_by_rid(&envelope.rid).await.unwrap();
        assert_eq!(
            RB.remove_by_column::<model::Envelope, _>("rid", &envelope.rid).await.unwrap(),
            1
        );
    }
}

