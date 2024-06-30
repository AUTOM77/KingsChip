use fantoccini::{ClientBuilder, Client, Locator};
use serde_json::{json, Value};

async fn _answer(client: &Client, questions: Vec<String>) -> Result<(),  Box<dyn std::error::Error>> {
    client.wait().for_element(Locator::Css(".QuestionBody")).await?;
    for q in questions{
        let qid = &format!("#QID{}-label", q);
        // println!("{:?}", qid);
        client.wait().for_element(Locator::Css(qid)).await?;
        if let Ok(elem) = client.find(Locator::Css(qid)).await {
            let _ = client.execute(
                "arguments[0].click();",
                vec![serde_json::to_value(elem)?],
            ).await;
        }
    }

    client.wait().for_element(Locator::Css("#NextButton")).await?;
    client.find(Locator::Css("#NextButton")).await?.click().await?;
    Ok(())
}

async fn call(drive: &str, url: &str, _code: &str) -> Result<(),  Box<dyn std::error::Error>> {
    let client = ClientBuilder::rustls()
        .capabilities(
            serde_json::json!({
                "moz:firefoxOptions": {
                    "args": ["--headless"]
                }
            })
            .as_object()
            .unwrap()
            .clone(),
        )
        .connect(drive).await.expect("failed to connect to WebDriver");
    client.goto(url).await?;
    client.find(Locator::Css(".Home_iframe__T3nfU")).await?.enter_frame().await?;

    client.wait().for_element(Locator::Css("#Questions")).await?;
    let f = client.form(Locator::Css("#Questions")).await?;
    f.set_by_name("QR~QID65~6~TEXT", _code).await?;

    if let Ok(elem) = client.find(Locator::Css("#NextButton")).await {
        let _ = client.execute(
            "arguments[0].click();",
            vec![serde_json::to_value(elem)?],
        ).await;
    }

    let data: Value = json!(
        [
            ["5-2"],
            ["7-2"],
            ["10-2"],
            ["60-1-2-col", "60-2-2-col", "60-3-2-col"],
            ["62-1-2-col", "62-2-2-col", "62-4-2-col"],
            ["63-2", "64-2"],
            ["30-2"],
            ["36-2"],
            ["71-1"],
            ["38-390"],
        ]
    );

    if let Some(array) = data.as_array() {
        for item in array.iter() {
            if let Some(inner_array) = item.as_array() {
                let items: Vec<String> = inner_array.iter()
                    .map(|x| x.as_str().unwrap_or("Invalid item").to_string())
                    .collect();
                _answer(&client, items).await?;
            }
        }
    }

    client.wait().for_element(Locator::Css("#QID45")).await?;
    client.wait().for_element(Locator::Css("#NextButton")).await?;
    if let Ok(elem) = client.find(Locator::Css("#NextButton")).await {
        let _ = client.execute(
            "arguments[0].click();",
            vec![serde_json::to_value(elem)?],
        ).await?;
    }
    
    client.wait().for_element(Locator::Css("#EndOfSurvey")).await?;
    let code = client.find(Locator::Css("strong")).await?.text().await?;

    println!("{:#?}", code);

    client.close().await?;
    Ok(())
}

pub fn interface(drive: &str, url: &str, code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    let _ = rt.block_on(call(drive, url, code));
    Ok(())
}
