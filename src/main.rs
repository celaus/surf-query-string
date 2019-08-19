#![feature(async_await)]

use surf::Exception;
use serde::Serialize;

#[derive(Serialize)]
struct MyGetParams {
    a: u64,
    b: String,
}

#[runtime::main]
async fn main() -> Result<(), Exception>  {
    let params = MyGetParams {
            a: 0x5ff,
            b: "https://blog.x5ff.xyz".into(),
    };

    let req =  surf::get("https://httpbin.org/get")
        .set_query(&params)?;
    let query_string = req.url().query();

    println!("Query string: {:?}", query_string);
    assert_eq!(Some("a=1535&b=https%3A%2F%2Fblog.x5ff.xyz"), query_string);

    println!("Response: {}", req
        .recv_string()
        .await?);
Ok(())
}
