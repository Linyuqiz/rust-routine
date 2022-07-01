use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // get
    get_example().await;

    // post
    post_method().await;
    post_form_example().await;
    post_json_example().await
}

async fn post_json_example() {
    // This will POST a body of `{"lang":"rust","body":"json"}`
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .json(&map)
        .send()
        .await
        .unwrap();
    println!("res = {:?}", res)
}

async fn post_form_example() {
    // This will POST a body of `foo=bar&baz=quux`
    let params = [("foo", "bar"), ("baz", "quux")];
    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .form(&params)
        .send()
        .await
        .unwrap();
    println!("res = {:?}", res)
}

async fn post_method() {
    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()
        .await
        .unwrap();
    println!("res = {:?}", res)
}

async fn get_example() {
    let body = reqwest::get("https://www.rust-lang.org")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("body = {:?}", body);
}
