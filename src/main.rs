use std::fs;
use std::io::prelude::*;
use serde::{ Serialize, Deserialize };
use fantoccini::*;
use fantoccini::error::*;
use webdriver::capabilities::Capabilities;
use serde_json::Value;
use regex::*;
use chrono::prelude::*;

#[derive(Serialize, Deserialize)]
struct Config {
    id       : String,
    password : String,
    headless : Option< bool >,
    no_future: Option< bool >,
    no_past  : Option< bool >,
    no_empty : Option< bool >,
    no_ok    : Option< bool >,
}

#[derive(Debug)]
struct Course {
    id  : String,
    name: String,
}

impl Config {
    fn read_from(file_name: &str) -> Config {
        let text: String = fs::read_to_string(file_name).expect(format!("{}을 찾을 수 없습니다.", file_name).as_str());
        toml::from_str(text.as_str()).expect(format!("{} 구성이 잘못된 것 같습니다.", file_name).as_str())
    }
}

fn o2b(o: Option< bool >) -> bool {
    o.is_some() && o.unwrap()
}

async fn get_client(headless: bool) -> Client {
    if headless {
        let mut caps = Capabilities::new();
        let chrome_opts = serde_json::json!({ "args": ["no-sandbox", "headless", "disable-gpu"] });
        caps.insert("goog:chromeOptions".to_string(), chrome_opts);
        Client::with_capabilities(format!("http://localhost:{}", 9515).as_str(), caps).await
    } else {
        Client::new(format!("http://localhost:{}", 9515).as_str()).await
    }.expect("WebDriver에 연결할 수 없습니다.")
}

async fn login(client: &mut Client, id: &String, password: &String) {
    client.goto("https://learn.hanyang.ac.kr/ultra/institution-page").await.unwrap();

    let portal_login = client.wait_for_find(Locator::Css("#entry-login-custom")).await.unwrap();
    *client = portal_login.click().await.unwrap();

    let mut uid = client.wait_for_find(Locator::Css("#uid")).await.unwrap();
    let mut upw = client.wait_for_find(Locator::Css("#upw")).await.unwrap();
    let login = client.wait_for_find(Locator::Css("#login_btn")).await.unwrap();

    uid.send_keys(id.as_str()).await.unwrap();
    upw.send_keys(password.as_str()).await.unwrap();
    *client = login.click().await.unwrap();
    client.current_url().await.expect("로그인에 실패했습니다.");
}

async fn get_user_id(client: &mut Client) -> String {
    client.goto("https://learn.hanyang.ac.kr/learn/api/v1/users/me").await.unwrap();
    let id_json_text = client.find(Locator::Css("pre")).await.unwrap().html(true).await.unwrap();
    let id_json: Value = serde_json::from_str(id_json_text.as_str()).unwrap();
    id_json[ "id" ].as_str().unwrap().to_string()
}

async fn get_course(client: &mut Client, user_id: &String) -> Vec< Course > {
    client.goto(format!("https://learn.hanyang.ac.kr/learn/api/v1/users/{}/memberships?expand=course.effectiveAvailability,course.permissions,courseRole&includeCount=true&limit=10000", user_id).as_str()).await.unwrap();
    let c_json_text = client.find(Locator::Css("pre")).await.unwrap().html(true).await.unwrap();
    let c_json: Value = serde_json::from_str(c_json_text.as_str()).unwrap();
    let ct = c_json[ "results" ].as_array().unwrap();
    let now_term = ct[ 0 ][ "course" ][ "term" ][ "id" ].as_str().unwrap();

    ct.iter().filter_map(|x| {
        if x[ "course" ][ "term" ][ "id" ].as_str()? == now_term {
            Some(Course {
                id  : x[ "course" ][ "id"   ].as_str()?.to_string(),
                name: x[ "course" ][ "name" ].as_str()?.to_string(),
            })
        } else { None }
    }).collect::< Vec< _ > >()
}

async fn attpr(client: &mut Client, config: &Config, course: &Vec< Course >) {
    let now = chrono::offset::Utc::now();
    let r = Regex::new(r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2})").unwrap();
    let no_future = o2b(config.no_future);
    let no_past   = o2b(config.no_past);
    let no_empty  = o2b(config.no_empty);
    let no_ok     = o2b(config.no_ok);

    for cs in course.iter() {
        let (mut P, mut F): (i32, i32) = (0, 0);
        let mut csv = Vec::new();

        client.goto(format!("https://learn.hanyang.ac.kr/webapps/blackboard/execute/blti/launchPlacement?blti_placement_id=_17_1&course_id={}&from_ultra=true", cs.id).as_str()).await.unwrap();

        if let Err(_) = client.find(Locator::Css("[class=\"noItems $emptyMsgCustomClass\"]")).await {
            let show_all = client.wait_for_find(Locator::Css("#listContainer_showAllButton")).await.unwrap();
            *client = show_all.click().await.unwrap();

            let mut rows = client.find_all(Locator::Css("#listContainer_databody > *")).await.unwrap();

            for row in rows.iter_mut() {
                let mut es = row.find_all(Locator::Css(".table-data-cell-value")).await.unwrap();
                let cs = es[ 1 ].html(true).await.unwrap();
                let mut ds = r.captures_iter(cs.as_str());
                let start = Utc.datetime_from_str(&ds.next().unwrap()[ 0 ], "%Y-%m-%d %H:%M").unwrap();
                let   end = Utc.datetime_from_str(&ds.next().unwrap()[ 0 ], "%Y-%m-%d %H:%M").unwrap();
                let    PF = es[ 6 ].html(true).await.unwrap();

                if (no_past && end < now) || (no_future && now < start) { continue }

                if PF.as_str() == "P" {
                    P += 1;
                } else {
                    F += 1;
                    csv.push(cs);
                }
            }
        }

        if (no_empty && P + F == 0) || (no_ok && F == 0) {
            continue;
        }

        println!("<{}> [{} / {}] {} ", cs.name, P, P + F, if F == 0 { "✔️" } else { "❌" });

        for nc in csv {
            println!("    {} 🔥 ", nc);
        }
    }

    println!("출결 확인을 완료했습니다!");
    std::io::stdin().read(&mut [0]).unwrap();
}

#[tokio::main]
async fn main() -> Result< (), CmdError > {
    let config = Config::read_from("config.toml");
    let mut client = get_client(config.headless.is_none() || config.headless.unwrap()).await;

    client.set_window_rect(0, 0, 1280, 1280).await?;
    login(&mut client, &config.id, &config.password).await;

    let user_id = get_user_id(&mut client).await;
    let course = get_course(&mut client, &user_id).await;

    attpr(&mut client, &config, &course).await;
    client.close().await
}