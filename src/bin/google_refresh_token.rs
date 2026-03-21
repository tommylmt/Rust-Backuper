use std::io;
use urlencoding::encode;

/**
 * One time usage to only get a refresh token, will not be deployed and is not part of the final project
 * Usage:
 *   $ CLIENT_ID=MY_CLIENT_ID CLIENT_SECRET=MY_CLIENT_SECRET cargo run --bin google_refresh_token
 */
fn main() {
    let client_id = env!("CLIENT_ID");
    let client_secret = env!("CLIENT_SECRET");
    let redirect_uri = "http://localhost:8880";

    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/auth?client_id={}&redirect_uri={}&response_type=code&scope=https://www.googleapis.com/auth/drive&access_type=offline&prompt=consent",
        client_id,
        encode(redirect_uri)
    );

    println!("Visit this URL in your browser:\n\n{auth_url}\n");
    println!("Paste the code here:");

    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();
    let code = code.trim();

    // Step 2: Exchange the code for tokens
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", code),
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("redirect_uri", redirect_uri),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .unwrap();

    let json: serde_json::Value = response.json().unwrap();
    println!("\nYour refresh token (save this in your config):\n{}", json["refresh_token"]);
}