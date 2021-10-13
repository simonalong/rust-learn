
use sqlx::MySqlConnection;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

#[async_std::main]
// or #[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = MySqlConnection::new()
        .max_connections(5)
        .connect("mysql://neo_test:neo@Test123@127.0.0.1:3306/neo?useUnicode=true&characterEncoding=UTF-8&useSSL=false&&allowPublicKeyRetrieval=true").await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);

    Ok(())
}

// #[macro_use]
// extern crate rbatis;

// use rbatis::crud::CRUD;

// /// may also write `CRUDTable` as `impl CRUDTable for BizActivity{}`
// /// #[crud_table]
// /// #[crud_table(table_name:biz_activity)]
// /// #[crud_table(table_name:"biz_activity"|table_columns:"id,name,version,delete_flag")]
// /// #[crud_table(table_name:"biz_activity"|table_columns:"id,name,version,delete_flag"|formats_pg:"id:{}::uuid")]
// #[crud_table(table_name:demo1)]
// #[derive(Clone, Debug)]
// pub struct BizActivity {
//   pub id: Option<String>,
//   pub name: Option<String>,
// }

// #[tokio::main]
// async fn main() {
//   /// enable log crate to show sql logs
//   fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);

//   /// initialize rbatis. May use `lazy_static` crate to define rbatis as a global variable because rbatis is thread safe
//   let rb = Rbatis::new();
//   /// connect to database  
//   rb.link("jdbc:mysql://127.0.0.1:3306/demo1?useUnicode=true&characterEncoding=UTF-8&useSSL=false&&allowPublicKeyRetrieval=true").await.unwrap();

//   /// customize connection pool parameters (optional)
// // let mut opt =PoolOptions::new();
// // opt.max_size=100;
// // rb.link_opt("mysql://root:123456@localhost:3306/test",&opt).await.unwrap();
//   /// newly constructed wrapper sql logic
//   /// 
//   let wrapper = rb.new_wrapper()
//           .eq("id", 1)                    //sql:  id = 1
//           .and()                          //sql:  and 
//           .ne("id", 1)                    //sql:  id <> 1
//           .in_array("id", &[1, 2, 3])     //sql:  id in (1,2,3)
//           .not_in("id", &[1, 2, 3])       //sql:  id not in (1,2,3)
//           .like("name", 1)                //sql:  name like 1
//           ;

//   let activity = BizActivity {
//     id: Some("12312".to_string()),
//     name: None,
//   };
//   /// saving
//   rb.save(&activity,&[]).await;
// //Exec ==> INSERT INTO biz_activity (create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version) VALUES ( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? )

//   /// batch saving
//   rb.save_batch(&vec![activity],&[]).await;
// //Exec ==> INSERT INTO biz_activity (create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version) VALUES ( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? ),( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? )

//   /// The query, Option wrapper, is None if the data is not found
//   let result: Option<BizActivity> = rb.fetch_by_column("id",&"1".to_string()).await.unwrap();
// //Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id =  ? 

//   /// query all
//   let result: Vec<BizActivity> = rb.list().await.unwrap();
// //Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1

//   ///query by id vec
//   let result: Vec<BizActivity> = rb.list_by_column("id",&["1".to_string()]).await.unwrap();
// //Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id IN  (?) 

//   ///query by wrapper
//   let r: Result<Option<BizActivity>, Error> = rb.fetch_by_wrapper( rb.new_wrapper().eq("id", "1")).await;
// //Query ==> SELECT  create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id =  ? 

//   ///delete
//   rb.remove_by_column::<BizActivity,_>("id", &"1".to_string()).await;
// //Exec ==> UPDATE biz_activity SET delete_flag = 0 WHERE id = 1

//   ///delete batch
//   rb.remove_batch_by_column::<BizActivity,_>("id", &["1".to_string(), "2".to_string()]).await;
// //Exec ==> UPDATE biz_activity SET delete_flag = 0 WHERE id IN (  ?  ,  ?  ) 

//   ///update
//   let mut activity = activity.clone();
//   let r = rb.update_by_column("id", &activity).await;  
// //Exec   ==> update biz_activity set  status = ?, create_time = ?, version = ?, delete_flag = ?  where id = ?
//   rb.update_by_wrapper( &activity, rb.new_wrapper().eq("id", "12312"), &[Skip::Value(&serde_json::Value::Null), Skip::Column("id")]).await;
// //Exec ==> UPDATE biz_activity SET  create_time =  ? , delete_flag =  ? , status =  ? , version =  ?  WHERE id =  ? 
// }