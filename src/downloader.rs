use regex::Regex;
use reqwest::header::USER_AGENT;
use rustube::{block, Id, VideoDetails, VideoFetcher};
use unescape::unescape;
use url::Url;

use crate::{
    extras::{remove_quotes, stripslashes},
    types::DownloadableVideo,
};

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
                    title: remove_quotes(&json["graphql"]["shortcode_media"]["id"].to_string()),
                    views: json["graphql"]["shortcode_media"]["video_view_count"].as_i64(),
                    description: remove_quotes(
                        &json["graphql"]["shortcode_media"]["edge_media_to_caption"]["edges"][0]
                            ["node"]["text"]
                            .to_string(),
                    ),
                    thumbnail: Some(remove_quotes(
                        &json["graphql"]["shortcode_media"]["thumbnail_src"].to_string(),
                    )),
                    url: Some(remove_quotes(
                        &json["graphql"]["shortcode_media"]["video_url"].to_string(),
                    )),
                    duration: None,
                    uploader: remove_quotes(
                        &json["graphql"]["shortcode_media"]["owner"]["username"].to_string(),
                    ),
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
            let txt = res.text().unwrap();
            println!("t: {}", txt);
            let txt_fixed = stripslashes(&txt).expect("stripslashes failed");
            //let json = res.json::<serde_json::Value>().unwrap();
            //let json: serde_json::Value = serde_json::from_str(&txt_fixed).unwrap();
            let json: serde_json::Value = serde_json::from_str(&txt).unwrap();
            //println!("{}", serde_json::to_string(&json).unwrap());
            let video_data = &json["aweme_list"][0];
            let um = &video_data["aweme_id"].to_string();
            println!("{}", um);
            //println!("{}", unescape(um).unwrap());
            //println!(
            //    "{}",
            //    remove_quotes(&video_data["video"]["play_addr"]["url_list"][0].to_string())
            //);
            let end_result = DownloadableVideo {
                title: remove_quotes(um),
                description: remove_quotes(&video_data["desc"].to_string()),
                views: video_data["statistics"]["play_count"].as_i64(),
                thumbnail: Some(remove_quotes(
                    &video_data["video"]["origin_cover"]["url_list"][0].to_string(),
                )),
                url: Some(remove_quotes(
                    &video_data["video"]["play_addr"]["url_list"][0].to_string(),
                )),
                duration: None,
                uploader: remove_quotes(&video_data["author"]["nickname"].to_string()),
            };
            println!("{:?}", end_result);
            return Ok(end_result);
        }
    }
}

pub fn yt(x: String) -> Result<DownloadableVideo, String> {
    //let id = Id::from_string(x);
    match Url::parse(&x) {
        Ok(x) => match VideoFetcher::from_url(&x) {
            Ok(x) => match block!(x.fetch()) {
                Ok(x) => {
                    let details = x.video_details();
                    let end_result = Ok(DownloadableVideo {
                        title: x.video_title().to_string(),
                        description: details.short_description.clone(),
                        views: Some(TryInto::<i64>::try_into(details.view_count).unwrap()),
                        thumbnail: Some(details.thumbnails[0].url.clone()),
                        url: None,
                        duration: None,
                        uploader: details.author.clone(),
                    });
                    println!("{:?}", end_result);
                    end_result
                }
                Err(err) => Err(err.to_string()),
            },
            Err(err) => Err(err.to_string()),
        },
        Err(err) => Err(err.to_string()),
    }
}
