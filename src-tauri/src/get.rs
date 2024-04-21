use serde_json::Value;

const KEY: &str = "AIzaSyAmD2JHQ3oFyHgnipQvtRWnO1_LN7A7cC8";

#[tauri::command]
pub async fn get_views(raw_video_id: &str) -> Result<u128, ()> {
    static APP_USER_AGENT: &str = concat!(
        env!("CARGO_PKG_NAME"),
        "/",
        env!("CARGO_PKG_VERSION")
    );
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .expect("error while building client");

    // https://www.youtube.com/watch?v=7lCDEYXw3mM
    let video_id = &raw_video_id[32..];

    // https://www.googleapis.com/youtube/v3/videos?id=7lCDEYXw3mM&key=YOUR_API_KEY&part=snippet,statistics&fields=items(id,snippet,statistics)
    let response = client
        .get(format!("https://www.googleapis.com/youtube/v3/videos?id={}&key={}&part={}&fields={}",
            video_id,
            KEY,
            "statistics",
            "items(statistics(viewCount))")
        )
        .send()
        .await
        .expect("error while sending request");

    let status = response.status();
    let response = response
        .text()
        .await
        .expect("error while getting text");
    println!("{}: {}",
        status,
        response);

    let json: Value = serde_json::from_str(&response).unwrap();
    let views = json["items"][0]["statistics"]["viewCount"]
        .to_string();
    let views = &views[1..views.len()-1];
    let views = views
        .parse::<u128>()
        .unwrap();
    println!("Views: {}", views);

    Ok(views)
}
