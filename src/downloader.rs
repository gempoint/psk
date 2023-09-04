use regex::Regex;
use reqwest::header::USER_AGENT;

use crate::types::DownloadableVideo;

pub fn insta(x: String) -> Result<DownloadableVideo, String> {
    // translated code from https://github.com/IshanJaiswal99/instagram-get-url/blob/master/src/index.js
    let binding = x.replace(" ", "");
    let ig_check = Regex::new(r"https?://(www\.)?instagram\.com/([A-Za-z0-9_]+)/?").unwrap();
    if !ig_check.is_match(&binding) {
        return Err(String::from("Invalid Instagram URL"));
    }
    let id = x.split("/").filter(|x| x.len() > 0).collect::<Vec<&str>>()[3];
    let client = reqwest::blocking::Client::new();
    let url = client.get(format!("https://www.instagram.com/p/{}/?utm_source=ig_web_copy_link?&__a=1&__d=1", id).as_str())
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36 Edg/107.0.1418.52")
        .header("sec-fetch-dest", "document")
        .header("sec-fetch-mode", "navigate")
        .header("sec-fetch-site", "none")
        .send();
    match url {
        Err(e) => return Err(e.to_string()),
        Ok(res) => {
            let json = res.json::<serde_json::Value>().unwrap();
            println!("{}", serde_json::to_string(&json).unwrap());
            if json["graphql"].is_object() {
                return Ok(DownloadableVideo {
                    title: json["graphql"]["shortcode_media"]["id"].to_string(),
                    views: json["graphql"]["shortcode_media"]["video_view_count"].as_i64(),
                    description: Some(
                        json["graphql"]["shortcode_media"]["edge_media_to_caption"]["edges"][0]
                            ["node"]["text"]
                            .to_string(),
                    ),
                    thumbnail: Some(
                        json["graphql"]["shortcode_media"]["thumbnail_src"].to_string(),
                    ),
                    url: json["graphql"]["shortcode_media"]["video_url"].to_string(),
                    duration: None,
                    uploader: json["graphql"]["shortcode_media"]["owner"]["username"].to_string(),
                });
            } else {
                return Err(format!("unknown error: {}", json["message"]));
            }
        }
    }
    //println!("{}", url.unwrap().text().unwrap())
}

pub fn tiktok(x: String) -> Result<DownloadableVideo, String> {
    // translated code from https://github.com/Prevter/tiktok-scraper/blob/main/src/index.ts
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36").build().unwrap();
    let idk = Regex::new(r"/^\d*$/").unwrap();
    //let re = Regex::new(r"/(vm|vt)\.tiktok\.com\/(.*)/").unwrap();
    let url_check = Regex::new(r"/(vm|vt)\.tiktok\.com\/(.*)/").unwrap();
    let video_check = Regex::new(r"/video/(\d*)").unwrap();

    println!("hi");
    let video_id = match idk.is_match(&x) {
        true => {
            println!("good video id");
            x.to_string()
        }
        false => {
            let mut tmp_url: String = x.to_string();
            if url_check.is_match(&x) {
                let res = client.get(x.clone()).send().map_err(|e| e.to_string())?;
                println!("checking video url");
                tmp_url = res.url().to_string();
            }

            match video_check.captures(tmp_url.as_str()) {
                Some(caps) => caps.get(1).unwrap().as_str().to_string(),
                None => return Err(format!("invalid url: {}", x)),
            }
        }
    };
    let url = client
        .get(format!(
            "https://api16-normal-v4.tiktokv.com/aweme/v1/feed/?aweme_id={}",
            video_id
        ))
        .send();
    match url {
        Err(e) => return Err(e.to_string()),
        Ok(res) => {
            println!("good");
            let json = res.json::<serde_json::Value>().unwrap();
            println!("{}", serde_json::to_string(&json).unwrap());
            let video_data = &json["aweme_list"][0];
            return Ok(DownloadableVideo {
                title: video_data["aweme_id"].to_string(),
                description: Some(video_data["desc"].to_string()),
                views: video_data["statistics"]["play_count"].as_i64(),
                thumbnail: Some(video_data["video"]["origin_cover"]["url_list"][0].to_string()),
                url: video_data["video"]["play_addr"]["url_list"][0].to_string(),
                duration: None,
                uploader: video_data["author"]["nickname"].to_string(),
            });
        }
    }
}
