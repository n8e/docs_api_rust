use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct JWT {
    token: String,
}

fn serialize<T: Serialize>(s: &T) -> String {
    serde_json::to_string(&s).unwrap()
}

fn deserialize<'de, T: Deserialize<'de>>(s: &'de String) -> T {
    serde_json::from_str(&s).unwrap()
}

async fn get_json(client: &Client, url: String, body: String) -> String {
    println!("Request::GET {}", url);
    let resp: String = client
        .get(url)
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    resp
}

async fn post_json(client: &Client, url: String, body: String) -> String {
    println!("Request::POST {}", url);
    let resp: String = client
        .post(url)
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    resp
}

#[cfg(test)]
mod tests {
    use rocket::tokio;
    use serde::Deserialize;
    use serde_json::json;
    use std::{
        thread,
        time::{self, Duration},
    };

    use crate::{
        models::user_model::User,
        tests::{deserialize, get_json, post_json, serialize, JWT},
    };

    #[tokio::test]
    async fn it_works() {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(1))
            .build()
            .unwrap();

        let one_sec = time::Duration::from_secs(1);
        thread::sleep(one_sec);

        println!("Request::GET http://localhost:8000/");
        let resp = client.get("http://localhost:8000/").send().await.unwrap();
        let body = resp.text().await.unwrap();

        assert_eq!(body, "\"Hello from rust and mongoDB\"".to_string());

        #[derive(Debug, Deserialize)]
        struct OID {
            #[serde(rename = "$oid")]
            oid: String,
        }

        #[derive(Debug, Deserialize)]
        struct ResponseBody {
            #[serde(rename = "insertedId")]
            inserted_id: OID,
        }

        let j = json!({
            "name":"Yannick",
            "location":"CH",
            "title":"Mr."
        });

        let new_user: User = serde_json::from_value(j).unwrap();

        let resp = post_json(
            &client,
            "http://localhost:8000/users".to_string(),
            serialize(&new_user),
        )
        .await;
        let v: ResponseBody = deserialize(&resp);
        use regex::Regex;
        let re = Regex::new(r"^[0-9a-z]{24}$").unwrap();
        assert!(re.is_match(&v.inserted_id.oid));

        let user_id = v.inserted_id.oid;
        let resp = get_json(
            &client,
            format!("http://localhost:8000/users/{}", &user_id),
            serialize(&new_user),
        )
        .await;
        let v = deserialize::<User>(&resp);
        assert_eq!(v.username.unwrap(), "yannick".to_string());

        let jwt_res = get_json(&client, "http://localhost:8000/auth/jwt".into(), "".into()).await;
        let jwt = deserialize::<JWT>(&jwt_res);
        let token = jwt.token;

        // delete client (using jwt token)
        let deleted: String = client
            .delete(format!("http://localhost:8000/users/{}", &user_id))
            .header("Authorization", token)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        assert_eq!(deleted, "\"User successfully deleted!\"".to_string());
    }
}