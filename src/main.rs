use std::fs;
use std::fs::File;
use std::io::prelude::*;
use serde::{ Serialize, Deserialize };
use fantoccini::*;
use fantoccini::error::*;
use webdriver::capabilities::Capabilities;

#[derive(Serialize, Deserialize)]
struct Course {
    id  : String,
    name: String,
    prof: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    port    : u16,
    id      : String,
    password: String,
    semester: String,
    course  : Option< Vec< Course > >,
}

impl Config {
    // toml 형식의 파일 io
    fn read_from(file_name: &str) -> Option< Config > {
        let text = fs::read_to_string(file_name).ok()?;
        toml::from_str(text.as_str()).ok()
    }

    fn write_to(&self, file_name: &str) -> Option< () > {
        let mut file = File::create(file_name).ok()?;
        file.write_all(toml::to_string(self).ok()?.as_bytes()).ok()
    }
}

#[tokio::main]
async fn main() -> Result< (), CmdError > {
    // config.toml 읽기
    let mut config =
        Config::read_from("config.toml").
        expect("failed to read \"config.toml\"");

    // 웹 드라이버 연결
    let mut caps = Capabilities::new();
    let chrome_opts = serde_json::json!({ "args": ["no-sandbox", "headless", "disable-gpu"] });
    caps.insert("goog:chromeOptions".to_string(), chrome_opts.clone());

    let mut client =
        Client::with_capabilities(format!("http://localhost:{}", config.port).as_str(), caps).await.
        // Client::new(format!("http://localhost:{}", config.port).as_str()).await.
        expect("Failed to connect to WebDriver");
    client.set_window_rect(0, 0, 1280, 1280).await?;

    // 블랙보드 접속
    client.goto("https://learn.hanyang.ac.kr/ultra/institution-page").await.
    expect("Failed to access Blackboard");

    // 로그인
    let portal_login =
        client.wait_for_find(Locator::Css("#entry-login-custom")).await?;
    client = portal_login.click().await?;

    let mut uid =
        client.wait_for_find(Locator::Css("#uid")).await?;
    uid.send_keys(config.id.as_str()).await?;

    let mut upw =
        client.wait_for_find(Locator::Css("#upw")).await?;
    upw.send_keys(config.password.as_str()).await?;

    let login =
        client.wait_for_find(Locator::Css("#login_btn")).await?;
    client = login.click().await?;

    // 코스 정보가 config.toml에 없으면 긁어와 저장하기
    if let None = config.course {
        // 전체 코스 페이지 접속
        client.goto("https://learn.hanyang.ac.kr/ultra/course").await?;

        // 현재 학기 페이지 접속
        let sem =
            client.wait_for_find(Locator::Css("[data-dropdown=\"courses-terms\"]")).await?;
        client = sem.click().await?;

        let now_sem =
            client.wait_for_find(Locator::Css(format!("[title=\"{}\"]", config.semester).as_str())).await.
            expect("Invalid semester");
        client = now_sem.click().await?;

        // 두 번째 보기
        let tw =
            client.wait_for_find(Locator::Css("[class=\"toggle-label input label-two js-label-toggle-grid\"]")).await?;
        client = tw.click().await?;

        // 코스 id, 과목명, 교수명 긁어오기
        let mut course = Vec::new();
        let mut ids = Vec::new();

        client.wait_for_find(Locator::Css("[data-course-id]")).await?;

        let course_blocks =
            client.find_all(Locator::Css("[data-course-id]")).await?;

        for elem in course_blocks.iter() {
            client.wait_for(|_| async move {
                let cid = elem.clone().attr("data-course-id").await.unwrap().unwrap();
                Ok(!cid.is_empty())
            }).await.unwrap();

            let id =
                elem.clone().attr("data-course-id").await?.unwrap();
            ids.push(id);
        }

        for id in ids {
            client.goto(format!("https://learn.hanyang.ac.kr/ultra/courses/{}/outline", id).as_str()).await?;

            let name =
                client.wait_for_find(Locator::Css("[data-ng-bind]")).await?.html(true).await?;
            let prof =
                client.wait_for_find(Locator::Css("[class=\"name ellipsis\"] > bb-username > bdi")).await?.html(true).await?;
            course.push(Course { id, name, prof });
        }

        // 파일로 저장
        config.course = Some(course);
        config.write_to("config.toml");
    }

    for Course { id, name, prof } in config.course.unwrap().iter() {
        // 코스 페이지 접속
        client.goto(format!("https://learn.hanyang.ac.kr/ultra/courses/{}/outline", id).as_str()).await?;

        // '도서 및 도구' 접속
        client.wait_for_find(Locator::Css(".element-card[bb-peek-sref][href]")).await?;

        let bs =
            client.find_all(Locator::Css(".element-card[bb-peek-sref][href]")).await?;
        let mut tools =
            bs.last().unwrap().clone();
        let tools_url =
            tools.attr("href").await?.unwrap();
        client.goto(tools_url.as_str()).await?;

        // '온라인 출석 조회' 접속
        client.wait_for_find(Locator::Css(".placement-link")).await?;

        let ls =
            client.find_all(Locator::Css(".placement-link")).await?;
        let cc = ls[ 1 ].clone();
        client = cc.click().await?;

        let (mut P, mut F): (i32, i32) = (0, 0);
        let mut ncs = Vec::new();

        // lti iframe 불러오기
        let lti =
            client.wait_for_find(Locator::Css("[title=\"LTI 실행\"]")).await?;
        client = lti.enter_frame().await?;

        // 강의가 없으면 건너뛰기
        if let Err(_) = client.find(Locator::Css("[class=\"noItems $emptyMsgCustomClass\"]")).await {
            // 모두 표시
            let show_all =
                client.wait_for_find(Locator::Css("#listContainer_showAllButton")).await?;
            client = show_all.click().await?;

            // 강의 목록
            let mut rows =
                client.find_all(Locator::Css("#listContainer_databody > *")).await?;

            // 출결 확인
            for row in rows.iter_mut() {
                let mut es = row.find_all(Locator::Css(".table-data-cell-value")).await?;

                match es[ 6 ].html(true).await?.as_str() {
                    "P" => P += 1,
                    "F" => {
                        F += 1;
                        ncs.push(es[ 1 ].html(true).await?);
                    },
                    _   => ()
                }
            }
        }

        println!("<{}({})> [{} / {}] {} ", name, prof, P, P + F, if F == 0 { "✔️" } else { "❌" });

        for nc in ncs {
            println!("    {} 🔥 ", nc);
        }
    }

    println!("출결 확인을 완료했습니다!");
    std::io::stdin().read(&mut [0]).unwrap();
    client.close().await
}