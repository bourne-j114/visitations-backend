use std::error::Error;
use csv::{Writer, Reader};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use scraper::{Html, Selector};
use std::path::Path;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use reqwest::header::COOKIE;
use std::env;
use dotenv::dotenv;
use tokio::time::{delay_for, Duration};
use reqwest::Client;
use crate::prisons::{Prisons, PrisonsMessage};
use crate::visitors::{Visitors, VisitorsMessage};
use crate::api_error::ApiError;

#[derive(Serialize, Deserialize, Default)]
struct Prison {
    prison_id: String,
    first_name: String,
    last_name: String,
    case: String,
    nation: String,
    gender: String,
    jail_date: String,
    visitors: Vec<Visitor>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Visitor {
    first_name: String,
    last_name: String,
    relation: String,
}

pub(crate) async fn import_family_and_friends() -> Result<(), Box<dyn Error>> {
    let path_str = env::var("FILE").expect("FILE not set");
    let file_name = format!("{}.csv", path_str);
    println!("FILE {}", &file_name);
    let mut rdr = Reader::from_path(file_name.clone().as_str())?;
    let cookie = env::var("COOKIE").expect("COOKIE not set");
    static APP_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36";
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .cookie_store(true)
        .build()?;

    for result in rdr.records() {
        let record = result?;
        let mut prison = Prison::default();

        prison.prison_id = record.get(1).unwrap().to_string();
        match Prisons::find(prison.prison_id.clone()){
            Ok(prison) => {
                println!("Prison id : {} exits", prison.prison_id);
            }
            Err(_) => {
                println!("Import prison id : {}", record.get(1).unwrap());
                let temp = record.get(3).unwrap().to_string();
                let split = temp.split(" ");
                let vec = split.collect::<Vec<&str>>();

                let first_name = vec[0].to_string();
                let last_name = vec[1].to_string();

                let gender = record.get(5).unwrap().to_string();
                let nation = record.get(7).unwrap().to_string();
                let case = record.get(11).unwrap().to_string();
                let jail_date = record.get(12).unwrap().to_string();
                prison.first_name = first_name;
                prison.last_name = last_name;
                prison.case = case;
                prison.gender = gender;
                prison.nation = nation;
                prison.jail_date = jail_date;
                delay_for(Duration::from_millis(2000)).await;
                //insert prison
                let mut prison_msg = PrisonsMessage::default();
                prison_msg.prison_id = prison.prison_id.clone();
                prison_msg.first_name = prison.first_name.clone();
                prison_msg.last_name = prison.last_name.clone();
                prison_msg.case_detail = prison.case.clone();
                let mut tmp = "f";
                if prison.gender.clone().as_str() == "ชาย" {
                    tmp = "m";
                }
                prison_msg.gender = tmp.to_string();
                prison_msg.jail_date = prison.jail_date.clone();
                prison_msg.location = "-".to_string();
                prison_msg.punish = "-".to_string();
                prison_msg.remark = "-".to_string();
                prison_msg.id_card = "-".to_string();
                 dbg!(&prison_msg);
                Prisons::create(prison_msg).unwrap();
                prison.visitors = get_family_and_friends(&client, &cookie, record.get(1).unwrap().to_string()).await?;
            }
        }

    }
    Ok(())
}

async fn login() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .connect_timeout(Duration::from_millis(5000))
        .build()?;
    let params = [("VIEWSTATE", ""), ("txtUserName", "31214"), ("txtPassword", "2730")];

    let url = format!("http://10.112.97.1/prism-search/Login.aspx?noRedirect=Y&MessageID=01");
    let res = client.post(&url)
        .form(&params)
        .send()
        .await?;
    Ok(())
}

async fn get_family_and_friends(client: &Client, cookie: &String, prison_id: String) -> Result<Vec<Visitor>, Box<dyn Error>> {

    delay_for(Duration::from_millis(500)).await;
    println!("setPrisoner");
    let url = format!("http://10.112.97.1/corrections/prisonSearch/setPrisoner/{}", prison_id);
    let _res = client.get(&url)
        .header(COOKIE, cookie.clone())
        .header("Upgrade-Insecure-Requests", "1")
        .send()
        .await?;

    delay_for(Duration::from_millis(100)).await;
    let temp = format!("1{}", &prison_id);
    let params = [("prisonerId", temp)];
    println!("GetDistPrisoner");
    let url = format!("http://10.112.97.1/corrections/ajax/GetDistPrisoner");
    let _res = client.post(&url)
        .header(COOKIE, cookie.clone())
        .header("X-Requested-With", "XMLHttpRequest")
        .timeout(Duration::from_millis(500))
        .form(&params)
        .send()
        .await?;
    delay_for(Duration::from_millis(100)).await;
    println!("postLogActivity");
    let url = format!("http://10.119.60.17:8000/postLogActivity");
    let _res = client.post(&url)
        .header(COOKIE, cookie.clone())
        .body("{\"LOG_USERID\" : \"15\",\"LOG_USERNAME\" : \"31214\",\"LOG_URL\" : \"prisonSearch/index\",\"LOG_PAGE\" : \"เลือกผู้ต้องขัง\",\"LOG_ACTIVITY\" : \"ค้นหาข้อมูล\",\"LOG_IP\" : \"10.112.97.21\",\"LOG_USER_FNAME\" : \"เอกสิทธิ์\",\"LOG_USER_LNAME\" : \"เถาว์ชาลี 2\",\"LOG_PRISON_CODE\" : \"312\",\"LOG_PARAMETER\" : {\"PRISONER_ID\":\"6331201961\",\"PRISON_FNAME\":\"\",\"REC_DATE\":\"\",\"SEARCH_FILTER\":\"1\",\"PRISON_LNAME\":\"\"},\"LOG_FLAG_VB\" : \"no\",\"LOG_TYPE\" : \"prison\"}:")
        .send()
        .await?;
    delay_for(Duration::from_millis(100)).await;
    let params = [("this_roles", "9"), ("action", "prisonRecord/prisonerHistory")];
    println!("getPermission");
    let url = format!("http://10.112.97.1/corrections/ajaxPermission/getPermission");
    let _res = client.post(&url)
        .header(COOKIE, cookie.clone())
        .header("X-Requested-With", "XMLHttpRequest")
        .form(&params)
        .send()
        .await?;
    delay_for(Duration::from_millis(500)).await;
    println!("familyAndFriends");
    let url = format!("http://10.112.97.1/corrections/prisonRecord/familyAndFriends");
    let mut res = client.get(&url)
        .header(COOKIE, cookie)
        .header("Upgrade-Insecure-Requests", "1")
        .send()
        .await?;
    let body = res.text().await?;


    delay_for(Duration::from_millis(100)).await;
    let temp = format!("2{}", &prison_id);
    let params = [("prisonerId", temp)];

    println!("GetDistPrisoner");
    let url = format!("http://10.112.97.1/corrections/ajax/GetDistPrisoner");
    let _res = client.post(&url)
        .header(COOKIE, cookie.clone())
        .header("X-Requested-With", "XMLHttpRequest")
        .form(&params)
        .send()
        .await?;

    delay_for(Duration::from_millis(100)).await;
    let params = [("this_roles", "9"), ("action", "prisonRecord/familyAndFriends")];
    println!("getPermission");
    let url = format!("http://10.112.97.1/corrections/ajaxPermission/getPermission");
    let _res = client.post(&url)
        .header(COOKIE, cookie.clone())
        .header("X-Requested-With", "XMLHttpRequest")
        .form(&params)
        .send()
        .await?;

    //  println!("body {}",  &body);
    let fragment = Html::parse_fragment(body.as_str());
    let tb_selector = Selector::parse("table[class=\"table table-bordered table-condensed table-striped\"]").unwrap();

    let tr_selector = Selector::parse("tr").unwrap();

    let table = fragment.select(&tb_selector).next().unwrap();
    let mut rs = vec![];
    let mut n = 0;
    for element in table.select(&tr_selector) {
        n += 1;
        if n > 1 {
            let td_selector_name = Selector::parse("td:nth-child(2)").unwrap();
            let el = element.select(&td_selector_name).next();
            if let Some(el) = el {
                let temp = el.inner_html();
                let split = temp.split(" ");
                let vec = split.collect::<Vec<&str>>();

                let first_name = vec[0].to_string();
                let last_name = vec[1].to_string();

                let td_selector_name = Selector::parse("td:nth-child(3)").unwrap();
                let el = element.select(&td_selector_name).next().unwrap();
                let relation = el.inner_html();

                let mut visitor_msg = VisitorsMessage {
                    gender: "-".to_string(),
                    prison_id: prison_id.clone(),
                    first_name,
                    last_name,
                    relations: relation,
                    phone_num: "-".to_string(),
                    line_id: "-".to_string(),
                    remark: "-".to_string(),
                };
                dbg!(&visitor_msg);
                Visitors::create(visitor_msg).unwrap();
            }
        }
    }
    Ok(rs)
}

async fn get_lock(client: &Client,prison_id: String)  -> Result<(), Box<dyn Error>>{
    let cookie = env::var("COOKIE").expect("COOKIE not set");
    let url = format!("http://10.112.97.1/prism-search/Prisoner/prisonerDetail.aspx?id={}",prison_id);
    let mut res = client.get(&url)
        .header(COOKIE, cookie)
        .header("Upgrade-Insecure-Requests", "1")
        .send()
        .await?;
    let body = res.text().await?;

    let fragment = Html::parse_fragment(body.as_str());
    let span_selector = Selector::parse("span[id=\"rePrisonerPunishDetail_Label106_0\"]").unwrap();

    let text = fragment.select(&span_selector).next().unwrap();
    println!("{:?}",text.text());
    Ok(())
}