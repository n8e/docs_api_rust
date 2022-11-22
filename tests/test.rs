// See https://rocket.rs/v0.4/guide/testing/#local-dispatching
#[cfg(test)]
mod test {
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;
    use rustlang_rocket_mongodb::rocket;

    #[test]
    fn get_users() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/users").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_user() {
        // Well get and post tests are identical ...
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/users")
            .header(ContentType::JSON)
            .body(r#"{ "name": "chacha" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let id = response.body_string().unwrap();
        let id: Vec<&str> = id.split("\"").collect();
        let mut response = client.get(format!("/users/{}", id[3])).dispatch();
        assert!(response.body().is_some());
        assert!(response.body_string().unwrap().contains(&id[3]));
        client.delete("/users").dispatch();
    }

    #[test]
    fn post_user() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/users")
            .header(ContentType::JSON)
            .body(r#"{ "name": "chacha" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let id = response.body_string().unwrap();
        let id: Vec<&str> = id.split("\"").collect();
        let mut response = client.get(format!("/users/{}", id[3])).dispatch();
        assert!(response.body().is_some());
        assert!(response.body_string().unwrap().contains(&id[3]));
        client.delete("/users").dispatch();
    }

    #[test]
    fn update_user() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/users")
            .header(ContentType::JSON)
            .body(r#"{ "name": "chacha" }"#)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_some());
        let id = response.body_string().unwrap();
        let id: Vec<&str> = id.split("\"").collect();
        let response = client
            .put(format!("/users/{}", id[3]))
            .header(ContentType::JSON)
            .body(r#"{ "name": "chichi" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let mut response = client.get(format!("/users/{}", id[3])).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_some());
        assert!(response.body_string().unwrap().contains("chichi"));
        client.delete("/users").dispatch();
    }

    #[test]
    fn delete_user() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/users")
            .header(ContentType::JSON)
            .body(r#"{ "name": "chacha" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let id = response.body_string().unwrap();
        let id: Vec<&str> = id.split("\"").collect();
        let mut response = client.delete(format!("/users/{}", id[3])).dispatch();
        assert!(response.body().is_some());
        assert!(response.body_string().unwrap().contains(&id[3]));
        client.delete("/users").dispatch();
    }

    #[test]
    fn delete_all() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        client.delete("/users").dispatch();
        let response = client
            .post("/users")
            .header(ContentType::JSON)
            .body(r#"{ "name": "chacha" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response = client.delete("/users").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}