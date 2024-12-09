
pub fn client() -> reqwest::Client {
    let client = reqwest::Client::builder() .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    client
}
