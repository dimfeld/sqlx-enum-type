use sqlx::PgPool;

#[derive(Debug, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
// Uncomment this for it to work
//#[sqlx(type_name = "text")]
enum SomeType {
    A,
    B,
    C,
}

#[derive(Debug, sqlx::FromRow)]
struct SomeStruct {
    enumdata: SomeType,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = PgPool::connect(std::env::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();
    let value = sqlx::query_as::<_, SomeStruct>("SELECT enumdata FROM abc")
        .fetch_one(&pool)
        .await
        .unwrap();
    println!("{value:?}");
}
