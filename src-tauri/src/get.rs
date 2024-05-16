use serde_json::Value;
use crate::utils::get_token;

#[tauri::command]
pub async fn get_views(raw_video_id: &str) -> Result<u128, String> {
    let token= match get_token() {
        Ok(string) => string,
        Err(_) => return Err("No token".to_string()),
    };

    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .expect("error while building client");

    if &raw_video_id[..32] != "https://www.youtube.com/watch?v=" {
        return Err("Incorrect video link".to_string());
    }
    let video_id = &raw_video_id[32..];

    let response = client
        .get(format!(
            "https://www.googleapis.com/youtube/v3/videos?id={}&key={}&part={}&fields={}",
            video_id, 
            token, 
            "statistics", 
            "items(statistics(viewCount))"
        ))
        .send()
        .await
        .expect("error while sending request");

    let status = response
        .status()
        .to_string();
    let response = response
        .text()
        .await
        .expect("error while getting text");
    println!("{}: {}", status, response);

    //Гавнокод ON
    if status != "200 OK" {
        return Err(status);
    }
    //Гавнокод OFF

    let json: Value = serde_json::from_str(&response).unwrap();
    let views = json["items"][0]["statistics"]["viewCount"].to_string();
    println!("{}", views);
    if views == r#""""# {
        return Err("No video statistics, maybe incorrect video ID".to_string());
    }
    let views = &views[1..views.len() - 1];
    let views = views.parse::<u128>().unwrap();
    println!("Views: {}", views);

    Ok(views)
}
