use chrono::Local;
use serde_json::value::Value;
use std::collections::HashMap;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Generate Theme Pattern：http://patorjk.com/software/taag/
    println!(
        r"
            ____                   ____                             __
           / __ \ ____ _ __  __   / __ \ ___   ____   ____   _____ / /_
          / / / // __ `// / / /  / /_/ // _ \ / __ \ / __ \ / ___// __/
         / /_/ // /_/ // /_/ /  / _, _//  __// /_/ // /_/ // /   / /_
        /_____/ \__,_/ \__, /  /_/ |_| \___// .___/ \____//_/    \__/
                      /____/               /_/
    "
    );

    let mut token = String::new();
    let mut param = String::new();
    let mut internal = String::new();
    println!("please input token：");
    std::io::stdin().read_line(&mut token).unwrap();
    println!("please input your param：");
    std::io::stdin().read_line(&mut param).unwrap();
    println!("please input execute internals time(second)：");
    std::io::stdin().read_line(&mut internal).unwrap();

    let mut count = 0;
    let number: u32 = match internal.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("input internal is wrong!"),
    };

    let mut interval = tokio::time::interval(Duration::from_secs(number as u64));
    println!("The day report progress starting:");
    loop {
        interval.tick().await;
        count = count + 1;
        if let Ok(result) = execute_report(token.parse().unwrap(), param.parse().unwrap()).await {
            println!("{:?} --- {:?} --- {:?}", Local::now(), result, count);
        }
    }
}

async fn execute_report(
    token: String,
    params: Value,
) -> Result<HashMap<String, Value>, reqwest::Error> {
    let return_status = reqwest::Client::new()
        .post("https://stu.eurasia.edu/yqsb/jkdj/save?token=".to_owned() + &*token)
        .json(&params)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?;

    Ok(return_status)
}
