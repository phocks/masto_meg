#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let client = megalodon::generator(
        megalodon::SNS::Mastodon,
        String::from("https://bne.social"),
        None,
        None,
    );

    let options = Some(megalodon::megalodon::GetTimelineOptions {
        only_media: None,
        limit: None,
        max_id: None,
        since_id: None,
        min_id: None,
    });

    let options_ref = options.as_ref();

    let res = client.get_local_timeline(options_ref).await?;

    // println!("{:?}", res);

    let json = serde_json::to_string(&res.json()).unwrap();
    println!("{}", json);

    // for status in res.json() {
    //     println!("{}: {}", status.account.username, status.content);
    // }

    Ok(())
}
