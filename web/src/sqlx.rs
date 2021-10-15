use sqlx::mysql::{MySqlPoolOptions, MySqlQueryResult};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://neo_test:neo@Test123@localhost:3306/demo1").await?;

    let row: (i64, ) = sqlx::query_as("SELECT ?")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    println!("{:?}", row);

    // 新增
    let sql = "insert into demo1(`name`, `group`) VALUES (?, ?)";
    let count:MySqlQueryResult = sqlx::query(sql).bind("name1").bind("group1").execute(&pool).await?;
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 33 }
    println!("{:?}", count);

    // 删除
    let sql = "delete from demo1 where `name` = ?";
    let count = sqlx::query(sql).bind("name1").execute(&pool).await?;
    // MySqlQueryResult { rows_affected: 9, last_insert_id: 0 }
    println!("{:?}", count);

    // 修改
    let sql = "update demo1 set `group` = ? where `name` = ?";
    let mut affect_rows = sqlx::query(sql).bind("group-chg").bind("name1").execute(&pool).await?;
    println!("{:?}", affect_rows);

    //
    // // 查询：一行
    // let sql = "select id,description,done from todos";
    // let mut cursor = sqlx::query(sql).fetch(&pool);
    // while let Some(row) = cursor.next().await? {
    //     let id: i32 = row.get(0);
    //     let description: String = row.get("description");
    //     let done: bool = row.get(2);
    //     println!("{},{}，{}", id, description, done);
    // }

    // 查询：多行

    // 查询：分页

    // 查询：一列

    // 查询：一个值

    // 查询：是否存在

    // 查询：个数


    Ok(())
}


// #[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
// pub struct Demo1 {
//     pub id: i64,
//     pub name: String,
// }
