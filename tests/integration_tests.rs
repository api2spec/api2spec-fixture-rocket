use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;

// Import the main rocket instance
use api2spec_fixture_rocket::rocket;

fn create_client() -> Client {
    Client::tracked(rocket()).expect("valid rocket instance")
}

mod health_tests {
    use super::*;

    #[test]
    fn test_health_endpoint_returns_ok() {
        let client = create_client();
        let response = client.get("/health").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let body = response.into_string().unwrap();
        assert!(body.contains("\"status\":\"ok\""));
        assert!(body.contains("\"version\":\"0.1.0\""));
    }

    #[test]
    fn test_health_ready_endpoint_returns_ready() {
        let client = create_client();
        let response = client.get("/health/ready").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let body = response.into_string().unwrap();
        assert!(body.contains("\"status\":\"ready\""));
        assert!(body.contains("\"version\":\"0.1.0\""));
    }
}

mod user_tests {
    use super::*;

    #[test]
    fn test_list_users_returns_users() {
        let client = create_client();
        let response = client.get("/users").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let body = response.into_string().unwrap();
        assert!(body.contains("Alice"));
        assert!(body.contains("Bob"));
        assert!(body.contains("alice@example.com"));
        assert!(body.contains("bob@example.com"));
    }

    #[test]
    fn test_get_user_by_id_returns_user() {
        let client = create_client();
        let response = client.get("/users/42").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let body = response.into_string().unwrap();
        assert!(body.contains("\"id\":42"));
        assert!(body.contains("Sample User"));
        assert!(body.contains("user@example.com"));
    }

    #[test]
    fn test_create_user_returns_created_user() {
        let client = create_client();
        let response = client
            .post("/users")
            .header(ContentType::JSON)
            .body(r#"{"id": 0, "name": "Charlie", "email": "charlie@example.com"}"#)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let body = response.into_string().unwrap();
        assert!(body.contains("\"id\":1"));
        assert!(body.contains("Charlie"));
        assert!(body.contains("charlie@example.com"));
    }

    #[test]
    fn test_create_user_with_invalid_json_returns_bad_request() {
        let client = create_client();
        let response = client
            .post("/users")
            .header(ContentType::JSON)
            .body(r#"{"invalid": json"#)
            .dispatch();

        // Rocket returns 400 Bad Request for invalid JSON
        assert_eq!(response.status(), Status::BadRequest);
    }

    #[test]
    fn test_create_user_with_missing_fields_returns_unprocessable() {
        let client = create_client();
        let response = client
            .post("/users")
            .header(ContentType::JSON)
            .body(r#"{"name": "NoEmail"}"#)
            .dispatch();

        // Rocket returns 422 Unprocessable Entity for missing required fields
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn test_update_user_returns_updated_user() {
        let client = create_client();
        let response = client
            .put("/users/5")
            .header(ContentType::JSON)
            .body(r#"{"id": 0, "name": "Updated Name", "email": "updated@example.com"}"#)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let body = response.into_string().unwrap();
        assert!(body.contains("\"id\":5"));
        assert!(body.contains("Updated Name"));
        assert!(body.contains("updated@example.com"));
    }

    #[test]
    fn test_update_user_with_invalid_json_returns_bad_request() {
        let client = create_client();
        let response = client
            .put("/users/5")
            .header(ContentType::JSON)
            .body(r#"not valid json"#)
            .dispatch();

        assert_eq!(response.status(), Status::BadRequest);
    }

    #[test]
    fn test_delete_user_returns_no_content() {
        let client = create_client();
        let response = client.delete("/users/10").dispatch();

        assert_eq!(response.status(), Status::NoContent);
    }

    #[test]
    fn test_get_user_posts_returns_posts() {
        let client = create_client();
        let response = client.get("/users/7/posts").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let body = response.into_string().unwrap();
        assert!(body.contains("\"user_id\":7"));
        assert!(body.contains("User Post"));
    }
}

mod post_tests {
    use super::*;

    #[test]
    fn test_list_posts_returns_posts() {
        let client = create_client();
        let response = client.get("/posts").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let body = response.into_string().unwrap();
        assert!(body.contains("First Post"));
        assert!(body.contains("Second Post"));
        assert!(body.contains("Hello world"));
        assert!(body.contains("Another post"));
    }

    #[test]
    fn test_get_post_by_id_returns_post() {
        let client = create_client();
        let response = client.get("/posts/99").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let body = response.into_string().unwrap();
        assert!(body.contains("\"id\":99"));
        assert!(body.contains("Sample Post"));
        assert!(body.contains("Post body"));
    }

    #[test]
    fn test_create_post_returns_created_post() {
        let client = create_client();
        let response = client
            .post("/posts")
            .header(ContentType::JSON)
            .body(r#"{"id": 0, "user_id": 5, "title": "New Post", "body": "Post content"}"#)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let body = response.into_string().unwrap();
        assert!(body.contains("\"id\":1"));
        assert!(body.contains("\"user_id\":5"));
        assert!(body.contains("New Post"));
        assert!(body.contains("Post content"));
    }

    #[test]
    fn test_create_post_with_invalid_json_returns_bad_request() {
        let client = create_client();
        let response = client
            .post("/posts")
            .header(ContentType::JSON)
            .body(r#"{"broken"#)
            .dispatch();

        assert_eq!(response.status(), Status::BadRequest);
    }

    #[test]
    fn test_create_post_with_missing_fields_returns_unprocessable() {
        let client = create_client();
        let response = client
            .post("/posts")
            .header(ContentType::JSON)
            .body(r#"{"title": "Missing user_id and body"}"#)
            .dispatch();

        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn test_update_post_returns_updated_post() {
        let client = create_client();
        let response = client
            .put("/posts/5")
            .header(ContentType::JSON)
            .body(r#"{"id": 0, "user_id": 3, "title": "Updated Title", "body": "Updated body"}"#)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let body = response.into_string().unwrap();
        assert!(body.contains("\"id\":5"));
        assert!(body.contains("\"user_id\":3"));
        assert!(body.contains("Updated Title"));
        assert!(body.contains("Updated body"));
    }

    #[test]
    fn test_update_post_with_invalid_json_returns_bad_request() {
        let client = create_client();
        let response = client
            .put("/posts/5")
            .header(ContentType::JSON)
            .body(r#"not valid json"#)
            .dispatch();

        assert_eq!(response.status(), Status::BadRequest);
    }

    #[test]
    fn test_update_post_with_id_over_100_returns_not_found() {
        let client = create_client();
        let response = client
            .put("/posts/101")
            .header(ContentType::JSON)
            .body(r#"{"id": 0, "user_id": 3, "title": "Test", "body": "Test body"}"#)
            .dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_delete_post_returns_no_content() {
        let client = create_client();
        let response = client.delete("/posts/10").dispatch();

        assert_eq!(response.status(), Status::NoContent);
    }

    #[test]
    fn test_delete_post_with_id_over_100_returns_not_found() {
        let client = create_client();
        let response = client.delete("/posts/101").dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }
}

mod error_tests {
    use super::*;

    #[test]
    fn test_nonexistent_route_returns_not_found() {
        let client = create_client();
        let response = client.get("/nonexistent").dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_invalid_user_id_type_returns_unprocessable_entity() {
        let client = create_client();
        // Rocket forwards when parameter guard fails, resulting in 422
        let response = client.get("/users/not-a-number").dispatch();

        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn test_invalid_post_id_type_returns_unprocessable_entity() {
        let client = create_client();
        // Rocket forwards when parameter guard fails, resulting in 422
        let response = client.get("/posts/abc").dispatch();

        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn test_post_to_users_without_content_type_still_works() {
        let client = create_client();
        // Rocket's Json extractor is lenient and accepts JSON even without explicit Content-Type
        let response = client
            .post("/users")
            .body(r#"{"id": 0, "name": "Test", "email": "test@example.com"}"#)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
    }

    #[test]
    fn test_method_not_allowed_on_health() {
        let client = create_client();
        let response = client.post("/health").dispatch();

        // Rocket returns 404 when no route matches (method mismatch)
        assert_eq!(response.status(), Status::NotFound);
    }
}
