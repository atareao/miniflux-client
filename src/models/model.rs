use serde::{Serialize, Deserialize};
use reqwest::Client;
use serde_json::{Value, json};
use tracing::debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    url: String,
    api_key: String,
    model: String,
    version: String,
    model_description: String,
    prompt: String,
    max_tokens: u32,
}

impl Model {
    pub fn new(url: String, api_key: String, model: String, version: String,
            model_description: String, prompt: String, max_tokens: u32) -> Self {
        Model {
            url,
            api_key,
            model,
            version,
            model_description,
            prompt,
            max_tokens,
        }
    }

    pub async fn process_news(&self, news: &Vec<Value>) -> Result<String, Box<dyn std::error::Error>> {
        debug!("Processing news...");
        let url = format!("{}/v1/messages", self.url);
        debug!("URL: {}", url);
        let content = json!({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "system": [
                {
                    "type": "text",
                    "text": &self.model_description
                },
                {
                    "type": "text",
                    "text": serde_json::to_string(news)?
                }
            ],
            "messages": [
                {
                    "role": "user",
                    "content": &self.prompt
                }
            ]
        });
        let client = Client::new();
        let response = client
            .post(&url)
            .header("content-type", "application/json")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", &self.version)
            .json(&content)
            .send()
            .await?;
        let response = response.json::<serde_json::Value>().await?;
        println!("Response: {:?}", response);
        //{"content": Array [Object {"text": String("
        let content = response
            .get("content")
            .ok_or("No content")?
            .as_array()
            .ok_or("No array")?
            .first()
            .ok_or("No first element")?
            .get("text")
            .ok_or("No text")?
            .as_str()
            .ok_or("No string")?;
        println!("Response: {:?}", content);
        debug!("Response: {:?}", content);
        Ok(content.to_string())
    }


    pub async fn process_news_from_feed(&self, news: &str) -> Result<String, Box<dyn std::error::Error>> {
        debug!("Processing news...");
        let url = format!("{}/v1/messages", self.url);
        debug!("URL: {}", url);
        let content = json!({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "system": [
                {
                    "type": "text",
                    "text": &self.model_description
                },
                {
                    "type": "text",
                    "text": news
                }
            ],
            "messages": [
                {
                    "role": "user",
                    "content": &self.prompt
                }
            ]
        });
        let client = Client::new();
        let response = client
            .post(&url)
            .header("content-type", "application/json")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", &self.version)
            .json(&content)
            .send()
            .await?;
        let response = response.json::<serde_json::Value>().await?;
        println!("Response: {:?}", response);
        //{"content": Array [Object {"text": String("
        let content = response
            .get("content")
            .ok_or("No content")?
            .as_array()
            .ok_or("No array")?
            .first()
            .ok_or("No first element")?
            .get("text")
            .ok_or("No text")?
            .as_str()
            .ok_or("No string")?;
        println!("Response: {:?}", content);
        debug!("Response: {:?}", content);
        Ok("".to_string())
    }
}

#[cfg(test)]
mod model_test {
    use super::Model;
    use dotenv::dotenv;
    use tracing::debug;

    #[tokio::test]
    async fn process_news() {
        dotenv().ok();
        let model = Model::new(
            std::env::var("MODEL_URL").expect("MODEL_URL is mandatory"),
            std::env::var("MODEL_API_KEY").expect("MODEL_API_KEY is mandatory"),
            std::env::var("MODEL_NAME").expect("MODEL_NAME is mandatory"),
            std::env::var("MODEL_VERSION").expect("MODEL_VERSION is mandatory"),
            std::env::var("MODEL_DESCRIPTION").expect("MODEL_DESCRIPTION is mandatory"),
            std::env::var("MODEL_PROMPT").expect("MODEL_PROMPT is mandatory"),
            std::env::var("MAX_TOKENS").expect("MAX_TOKENS is mandatory")
            .parse::<u32>().unwrap(),
        );
        let news = r#"
<?xml version="1.0" encoding="UTF-8"?><rss xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:content="http://purl.org/rss/1.0/modules/content/" xmlns:atom="http://www.w3.org/2005/Atom" version="2.0" xmlns:media="http://search.yahoo.com/mrss/">
    <channel>
        <title><![CDATA[BBC News]]></title>
        <description><![CDATA[BBC News - World]]></description>
        <link>https://www.bbc.co.uk/news/world</link>
        <image>
            <url>https://news.bbcimg.co.uk/nol/shared/img/bbc_news_120x60.gif</url>
            <title>BBC News</title>
            <link>https://www.bbc.co.uk/news/world</link>
        </image>
        <generator>RSS for Node</generator>
        <lastBuildDate>Thu, 15 May 2025 05:56:53 GMT</lastBuildDate>
        <atom:link href="https://feeds.bbci.co.uk/news/world/rss.xml" rel="self" type="application/rss+xml"/>
        <copyright><![CDATA[Copyright: (C) British Broadcasting Corporation, see https://www.bbc.co.uk/usingthebbc/terms-of-use/#15metadataandrssfeeds for terms and conditions of reuse.]]></copyright>
        <language><![CDATA[en-gb]]></language>
        <ttl>15</ttl>
        <item>
            <title><![CDATA[Beginning of the end? Ukraine's front-line soldiers eye Russia talks with hope]]></title>
            <description><![CDATA[There is cautious optimism in eastern Ukraine as talks are set to take place in Istanbul.]]></description>
            <link>https://www.bbc.com/news/articles/c8e6wj07xlno</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/c8e6wj07xlno#0</guid>
            <pubDate>Wed, 14 May 2025 21:09:37 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/e5c8/live/ea5a0120-30e0-11f0-8519-3b5a01ebe413.jpg"/>
        </item>
        <item>
            <title><![CDATA[UK tech tycoon Mike Lynch's yacht was toppled by 'extreme wind', report says]]></title>
            <description><![CDATA[Investigators say 80mph winds "violently" knocked over the vessel, causing it to flood within seconds.]]></description>
            <link>https://www.bbc.com/news/articles/c331djzymz6o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/c331djzymz6o#0</guid>
            <pubDate>Wed, 14 May 2025 23:40:56 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/a637/live/63cbb8c0-30fd-11f0-bd46-4b7602f31d2d.jpg"/>
        </item>
        <item>
            <title><![CDATA[At least 21 killed in crash in Mexico, official says]]></title>
            <description><![CDATA[Several other people have been injured and are receiving medical treatment, a local official says.]]></description>
            <link>https://www.bbc.com/news/articles/cp3nz3126xxo</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/cp3nz3126xxo#0</guid>
            <pubDate>Wed, 14 May 2025 23:35:28 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/fb6d/live/1ed37f50-312a-11f0-8519-3b5a01ebe413.jpg"/>
        </item>
        <item>
            <title><![CDATA[Mexican beauty influencer shot dead during TikTok livestream]]></title>
            <description><![CDATA[Valeria Marquez was filming herself in her beauty salon in Guadalajara when she was shot.]]></description>
            <link>https://www.bbc.com/news/articles/ckg4rn0z1r6o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/ckg4rn0z1r6o#0</guid>
            <pubDate>Thu, 15 May 2025 03:48:46 GMT</pubDate>
            <media:thumbnail width="240" height="134" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/cee0/live/0a784870-3138-11f0-9a1d-1132c823f273.jpg"/>
        </item>
        <item>
            <title><![CDATA[Construction sites appear in Gaza ahead of Israeli-US aid plan rejected by UN, images show]]></title>
            <description><![CDATA[Satellite images show building at sites in south Gaza after US says controversial Israeli aid plan to proceed.]]></description>
            <link>https://www.bbc.com/news/articles/ce82ene10p0o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/ce82ene10p0o#0</guid>
            <pubDate>Wed, 14 May 2025 23:13:37 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/2849/live/bce40690-309b-11f0-8947-7d6241f9fce9.jpg"/>
        </item>
        <item>
            <title><![CDATA[Trump's meeting with Sharaa, unthinkable just months ago, boosts Syrians' hopes]]></title>
            <description><![CDATA[Syria's interim leader met the US president in Riyadh after he announced he would lift sanctions on Syria.]]></description>
            <link>https://www.bbc.com/news/articles/c9dqp842nl8o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/c9dqp842nl8o#1</guid>
            <pubDate>Wed, 14 May 2025 18:00:25 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/370c/live/d15c0910-30e1-11f0-8519-3b5a01ebe413.jpg"/>
        </item>
        <item>
            <title><![CDATA[Drone attacks raise stakes in new phase of Sudan's civil war]]></title>
            <description><![CDATA[Experts tell the BBC the air strikes showcase the growth of advanced drone warfare in Africa.]]></description>
            <link>https://www.bbc.com/news/articles/cg4vz0qy77no</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/cg4vz0qy77no#1</guid>
            <pubDate>Wed, 14 May 2025 23:59:47 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/f778/live/4eca3160-30ab-11f0-a620-b132f633f1b3.jpg"/>
        </item>
        <item>
            <title><![CDATA[Trump's critics and supporters unite against Qatar plane deal]]></title>
            <description><![CDATA[The White House has doubled down on the proposed deal which has been criticised by some of the president's top supporters.]]></description>
            <link>https://www.bbc.com/news/articles/ckg4zk22n9wo</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/ckg4zk22n9wo#1</guid>
            <pubDate>Thu, 15 May 2025 00:03:05 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/897b/live/7fad7530-310f-11f0-a4e1-23b53059b70e.jpg"/>
        </item>
        <item>
            <title><![CDATA[Israel issues major evacuation order for Palestinians sheltering in Gaza City]]></title>
            <description><![CDATA[Israel alleges several buildings are being used by Hamas. Local authorities and aid agencies say thousands of civilians are sheltering there. ]]></description>
            <link>https://www.bbc.com/news/articles/c2d5x60pw7eo</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/c2d5x60pw7eo#1</guid>
            <pubDate>Wed, 14 May 2025 22:23:40 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/5f03/live/b46bb240-3110-11f0-a4e1-23b53059b70e.jpg"/>
        </item>
        <item>
            <title><![CDATA[Aussie Rules great dies using voluntary assisted dying laws]]></title>
            <description><![CDATA[Robert Walls - a legend of the Carlton Football Club - was diagnosed with cancer in 2023.]]></description>
            <link>https://www.bbc.com/news/articles/ce81yvn3gnro</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/ce81yvn3gnro#1</guid>
            <pubDate>Thu, 15 May 2025 03:37:16 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/cd5f/live/95e8b910-3135-11f0-9640-eb038908f012.jpg"/>
        </item>
        <item>
            <title><![CDATA[US academic released from immigration detention after judge's ruling]]></title>
            <description><![CDATA[The Indian national and postdoctoral fellow at Georgetown University was arrested by immigration agents in March. ]]></description>
            <link>https://www.bbc.com/news/articles/ceqgl3z0qvyo</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/ceqgl3z0qvyo#1</guid>
            <pubDate>Thu, 15 May 2025 02:06:44 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/b2e1/live/727cf800-312c-11f0-96e2-83a5b32e8e83.jpg"/>
        </item>
        <item>
            <title><![CDATA[European Commission wrong to deny release of von der Leyen messages, court says]]></title>
            <description><![CDATA[The Commission is found to have wrongly refused to publish texts from its president during the Covid pandemic.]]></description>
            <link>https://www.bbc.com/news/articles/cx2x7gzdr01o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/cx2x7gzdr01o#1</guid>
            <pubDate>Wed, 14 May 2025 13:48:09 GMT</pubDate>
            <media:thumbnail width="240" height="134" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/caf8/live/dc6adfc0-30b9-11f0-a3a4-4bc37b535b8c.jpg"/>
        </item>
        <item>
            <title><![CDATA[White South Africans going to US are cowards, Ramaphosa says]]></title>
            <description><![CDATA[South Africans are resilient and don't run away from their problems, President Ramaphosa says. ]]></description>
            <link>https://www.bbc.com/news/articles/ce82e67p8p2o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/ce82e67p8p2o#1</guid>
            <pubDate>Wed, 14 May 2025 09:37:22 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/6bc4/live/952ac080-3096-11f0-8519-3b5a01ebe413.jpg"/>
        </item>
        <item>
            <title><![CDATA[Peru's prime minister resigns ahead of no-confidence vote ]]></title>
            <description><![CDATA[Gustavo Adrianzén's resignation is another blow to President Dina Boluarte amid anger over rising crime.]]></description>
            <link>https://www.bbc.com/news/articles/c70nedjg1y1o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/c70nedjg1y1o#1</guid>
            <pubDate>Wed, 14 May 2025 11:10:22 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/200a/live/e2279810-30a8-11f0-a620-b132f633f1b3.jpg"/>
        </item>
        <item>
            <title><![CDATA[Judge cuts Menendez brothers' sentences giving them chance of freedom]]></title>
            <description><![CDATA[The decision could pave the way for Erik and Lyle Menendez to be freed, in a case that still divides the US. ]]></description>
            <link>https://www.bbc.com/news/articles/c2e3xnx471wo</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/c2e3xnx471wo#1</guid>
            <pubDate>Wed, 14 May 2025 03:44:25 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/bf25/live/44301980-30a7-11f0-96c3-cf669419a2b0.jpg"/>
        </item>
        <item>
            <title><![CDATA[Israeli strikes across Gaza kill 80, hospitals and rescuers say]]></title>
            <description><![CDATA[Hospitals say 29 children and 15 women were among the dead after the northern Jabalia area were hit overnight.]]></description>
            <link>https://www.bbc.com/news/articles/ceqglvlwzx5o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/ceqglvlwzx5o#1</guid>
            <pubDate>Wed, 14 May 2025 19:23:59 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/520e/live/166ad260-30a2-11f0-96c3-cf669419a2b0.jpg"/>
        </item>
        <item>
            <title><![CDATA[Human rights groups warn of 'surge' in migrant worker deaths in Saudi Arabia]]></title>
            <description><![CDATA[Workers are already dying from preventable workplace accidents as the country prepares to host the 2034 World Cup, reports say.]]></description>
            <link>https://www.bbc.com/news/articles/cgj86wqe0dlo</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/cgj86wqe0dlo#1</guid>
            <pubDate>Wed, 14 May 2025 08:49:49 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/a8cc/live/6b20ba60-3096-11f0-ad02-b95db8a5d645.jpg"/>
        </item>
        <item>
            <title><![CDATA[Scared and malnourished - footage from Gaza shows plight of children and aftermath of Israeli strike]]></title>
            <description><![CDATA[Israel says it targeted top Hamas militants hiding among civilians at the Khan Younis hospital.]]></description>
            <link>https://www.bbc.com/news/articles/cjdznz727z8o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/cjdznz727z8o#1</guid>
            <pubDate>Wed, 14 May 2025 02:38:34 GMT</pubDate>
            <media:thumbnail width="240" height="134" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/7994/live/9e040040-3073-11f0-8519-3b5a01ebe413.png"/>
        </item>
        <item>
            <title><![CDATA[Military rulers in Mali dissolve all political parties]]></title>
            <description><![CDATA[All political meetings have also been banned as the junta intensifies its grip on power. ]]></description>
            <link>https://www.bbc.com/news/articles/cqj7z92xw9no</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/cqj7z92xw9no#1</guid>
            <pubDate>Wed, 14 May 2025 14:30:20 GMT</pubDate>
            <media:thumbnail width="240" height="134" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/47d9/live/76b791b0-30ae-11f0-bb6f-6fd2e73bdb9e.jpg"/>
        </item>
        <item>
            <title><![CDATA[Plane crash victims' families file complaint against Jeju Air CEO]]></title>
            <description><![CDATA[A criminal complaint was filed against 15 people, including the transport minister, for negligence.]]></description>
            <link>https://www.bbc.com/news/articles/ckgqnk3q385o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/ckgqnk3q385o#1</guid>
            <pubDate>Wed, 14 May 2025 04:26:10 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/8d27/live/093c7050-3078-11f0-86ef-bb0c82c65c1f.jpg"/>
        </item>
        <item>
            <title><![CDATA[Uruguay's José Mujica, world's 'poorest president', dies]]></title>
            <description><![CDATA[The former guerrilla who governed Uruguay from 2010 to 2015 earned his nickname because of his modest lifestyle.]]></description>
            <link>https://www.bbc.com/news/articles/c0j71402z58o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/c0j71402z58o#1</guid>
            <pubDate>Wed, 14 May 2025 00:18:18 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/c871/live/51e9ec90-302a-11f0-9c4d-7b4728612d5d.jpg"/>
        </item>
        <item>
            <title><![CDATA[Céline Dion sends message to Eurovision as favourites Sweden qualify]]></title>
            <description><![CDATA[The singer sends a good luck message to Switzerland, as 10 countries qualify for Saturday's final.]]></description>
            <link>https://www.bbc.com/news/articles/c0mrvw1jezxo</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/c0mrvw1jezxo#1</guid>
            <pubDate>Tue, 13 May 2025 21:42:05 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/c960/live/aee38fa0-303e-11f0-96c3-cf669419a2b0.png"/>
        </item>
        <item>
            <title><![CDATA[Champion cyclist sentenced over Olympian wife's car death]]></title>
            <description><![CDATA[Melissa Hoskins died after she was struck by a car driven by her husband Rohan Dennis in 2023.]]></description>
            <link>https://www.bbc.com/news/articles/cx2jv71nd43o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/cx2jv71nd43o#1</guid>
            <pubDate>Wed, 14 May 2025 03:38:28 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/9c6c/live/598b1ee0-3072-11f0-8947-7d6241f9fce9.jpg"/>
        </item>
        <item>
            <title><![CDATA[Halle Bailey gets restraining order against 'abusive' ex]]></title>
            <description><![CDATA[She alleges rapper DDG has been violent towards her since their break-up. He has not yet commented.]]></description>
            <link>https://www.bbc.com/news/articles/cz70qgvjvv1o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/cz70qgvjvv1o#1</guid>
            <pubDate>Wed, 14 May 2025 10:47:47 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/8f0c/live/e0374880-30a2-11f0-96c3-cf669419a2b0.jpg"/>
        </item>
        <item>
            <title><![CDATA[Biden aide hits back at book alleging cover-up of his 'deterioration']]></title>
            <description><![CDATA[Biden's condition during his 2024 presidential campaign was said to be so poor that aides discussed putting him in a wheelchair.]]></description>
            <link>https://www.bbc.com/news/articles/c4g23zzl1gwo</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/c4g23zzl1gwo#1</guid>
            <pubDate>Wed, 14 May 2025 02:29:01 GMT</pubDate>
            <media:thumbnail width="240" height="134" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/0e6b/live/78e7d210-3054-11f0-86ed-cf9a6d65532a.jpg"/>
        </item>
        <item>
            <title><![CDATA[BBC joins Gaza children as they are evacuated to Jordan for treatment]]></title>
            <description><![CDATA[The BBC's Fergal Keane accompanies children being evacuated from Gaza on their journey to Jordan.]]></description>
            <link>https://www.bbc.com/news/articles/cg5v2rjp84ro</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/cg5v2rjp84ro#2</guid>
            <pubDate>Thu, 15 May 2025 05:44:20 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/0563/live/a1b035c0-312b-11f0-8947-7d6241f9fce9.png"/>
        </item>
        <item>
            <title><![CDATA[Why India could not stop IMF bailout to Pakistan]]></title>
            <description><![CDATA[As one of 25 member groups on the IMF board, India's influence at the fund is limited, say experts.]]></description>
            <link>https://www.bbc.com/news/articles/c3v5r1erdpgo</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/c3v5r1erdpgo#2</guid>
            <pubDate>Wed, 14 May 2025 23:48:46 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/de6d/live/a34ec470-3084-11f0-aa5f-71944dd67a64.jpg"/>
        </item>
        <item>
            <title><![CDATA[Meet the 'invisible crew' who have 35 seconds to prevent a Eurovision blunder]]></title>
            <description><![CDATA[Find out how Eurovision gets one act off the stage and another one in place in under a minute.]]></description>
            <link>https://www.bbc.com/news/articles/c1e679vn6yqo</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/c1e679vn6yqo#2</guid>
            <pubDate>Thu, 15 May 2025 00:10:44 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/0b95/live/3b886ab0-30c2-11f0-9228-d36f84fd1370.jpg"/>
        </item>
        <item>
            <title><![CDATA[Do Afrikaners want to take Trump up on his South African refugee offer?]]></title>
            <description><![CDATA[Claims of a white genocide are widely discredited, so how do Afrikaners feel about Trump's claims – and offer of refugee status?]]></description>
            <link>https://www.bbc.com/news/articles/cp3q129gpe4o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/cp3q129gpe4o#2</guid>
            <pubDate>Thu, 15 May 2025 01:43:02 GMT</pubDate>
            <media:thumbnail width="240" height="134" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/2ebf/live/a0a291c0-30e8-11f0-92b4-d99be905cc56.jpg"/>
        </item>
        <item>
            <title><![CDATA[What did Erik and Lyle Menendez do and when could they be released?]]></title>
            <description><![CDATA[On Tuesday, a Los Angeles judge reduced Lyle and Erik's murder sentence, making them eligible for parole. ]]></description>
            <link>https://www.bbc.com/news/articles/c5y2dgep4zzo</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/c5y2dgep4zzo#2</guid>
            <pubDate>Wed, 14 May 2025 11:20:29 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/5ff6/live/fe8e24a0-30a9-11f0-a620-b132f633f1b3.jpg"/>
        </item>
        <item>
            <title><![CDATA['Go back to Ukraine': War refugees complain of abuse in Poland]]></title>
            <description><![CDATA[Dozens say they have been abused in public, bullied in schools and seen increasing xenophobic material online.]]></description>
            <link>https://www.bbc.com/news/articles/clygv2dj78zo</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/clygv2dj78zo#2</guid>
            <pubDate>Wed, 14 May 2025 05:00:07 GMT</pubDate>
            <media:thumbnail width="240" height="134" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/a7ef/live/272c9cd0-2ffa-11f0-8519-3b5a01ebe413.jpg"/>
        </item>
        <item>
            <title><![CDATA[The heartbeat of a billion: What Virat Kohli meant to India]]></title>
            <description><![CDATA[Legendary India batter Virat Kohli announced retirement from international Test cricket on Monday. ]]></description>
            <link>https://www.bbc.com/news/articles/cp857gryxn1o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/cp857gryxn1o#2</guid>
            <pubDate>Tue, 13 May 2025 23:16:31 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/bcfb/live/204bae40-2fa7-11f0-8f57-b7237f6a66e6.jpg"/>
        </item>
        <item>
            <title><![CDATA[El Chapo & Deputy Jesus - why Kenya's president has so many nicknames]]></title>
            <description><![CDATA[William Ruto has the uncanny ability to attract numerous alternative handles, most are unflattering.]]></description>
            <link>https://www.bbc.com/news/articles/cewdl20d8lxo</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/cewdl20d8lxo#2</guid>
            <pubDate>Tue, 13 May 2025 23:31:41 GMT</pubDate>
            <media:thumbnail width="240" height="134" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/c39d/live/d836c570-2bf7-11f0-93a5-5159a2167ded.jpg"/>
        </item>
        <item>
            <title><![CDATA[How real is the risk of nuclear war between India and Pakistan?]]></title>
            <description><![CDATA[Recent hostilities were a reminder of how easily a crisis could escalate into catastrophe.]]></description>
            <link>https://www.bbc.com/news/articles/c2e373yzndro</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/c2e373yzndro#2</guid>
            <pubDate>Tue, 13 May 2025 23:35:23 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/b97d/live/d6878360-2f5b-11f0-b26b-ab62c890638b.jpg"/>
        </item>
        <item>
            <title><![CDATA[Is Trump allowed to accept $400m luxury plane as a gift?]]></title>
            <description><![CDATA[The US leader has faced criticism over his plan to accept a plane from Qatar worth an estimated $400m.]]></description>
            <link>https://www.bbc.com/news/articles/cx2eylpdg9po</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/cx2eylpdg9po#2</guid>
            <pubDate>Tue, 13 May 2025 17:29:35 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/53d7/live/a583e7d0-301f-11f0-8519-3b5a01ebe413.jpg"/>
        </item>
        <item>
            <title><![CDATA[Gold is booming - but investors lured in by the hype could lose out, warn experts]]></title>
            <description><![CDATA[Trade wars and volatile markets have contributed to a gold rush, but investors shouldn't put all their eggs in this one basket, warn experts]]></description>
            <link>https://www.bbc.com/news/articles/c5ygyjy7kz5o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/c5ygyjy7kz5o#2</guid>
            <pubDate>Tue, 13 May 2025 01:02:20 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/aae4/live/82babf60-2be4-11f0-8f57-b7237f6a66e6.png"/>
        </item>
        <item>
            <title><![CDATA[Why the mighty Himalayas are getting harder and harder to see]]></title>
            <description><![CDATA[Severe air pollution is obscuring visibility of the highest mountain peaks on Earth, hitting tourism hard.]]></description>
            <link>https://www.bbc.com/news/articles/cvgdl092jj5o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/cvgdl092jj5o#2</guid>
            <pubDate>Tue, 13 May 2025 01:20:27 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/96b1/live/9f0490d0-2e85-11f0-8ff1-59f5dcf8e9f5.jpg"/>
        </item>
        <item>
            <title><![CDATA[Stars hit the Cannes Film Festival: Five things to look out for]]></title>
            <description><![CDATA[Scarlett Johansson, Paul Mescal and Tom Cruise star in films that will receive their premieres.]]></description>
            <link>https://www.bbc.com/news/articles/c0j78v19gp2o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/c0j78v19gp2o#2</guid>
            <pubDate>Tue, 13 May 2025 00:46:47 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/23c8/live/2a046a60-2ff9-11f0-96c3-cf669419a2b0.jpg"/>
        </item>
        <item>
            <title><![CDATA[Trump's mediation offer on Kashmir puts India in a tight spot]]></title>
            <description><![CDATA[For decades, third-party mediation on Kashmir has been taboo in the Indian foreign ministry.]]></description>
            <link>https://www.bbc.com/news/articles/cjwq0y7d14lo</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/cjwq0y7d14lo#2</guid>
            <pubDate>Tue, 13 May 2025 03:00:23 GMT</pubDate>
            <media:thumbnail width="240" height="134" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/3e82/live/d612c430-2f3d-11f0-8ff1-59f5dcf8e9f5.png"/>
        </item>
        <item>
            <title><![CDATA[China has come to the table - but this fight is far from over]]></title>
            <description><![CDATA[Beijing can claim its defiance has delivered a battle victory in the trade war.]]></description>
            <link>https://www.bbc.com/news/articles/c1j5r9ngd3ko</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/c1j5r9ngd3ko#2</guid>
            <pubDate>Mon, 12 May 2025 13:38:52 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/ccfa/live/cc45b290-2f34-11f0-8bc1-7df2ca6d41e7.png"/>
        </item>
        <item>
            <title><![CDATA[What does the US-China tariff deal mean?]]></title>
            <description><![CDATA[The agreement marks a major de-escalation of the trade war between the world's two biggest economies.]]></description>
            <link>https://www.bbc.com/news/articles/clyg2939ej2o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/clyg2939ej2o#2</guid>
            <pubDate>Mon, 12 May 2025 16:10:13 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/a75d/live/a4cd6290-2f2e-11f0-9869-cde5df3646f9.jpg"/>
        </item>
        <item>
            <title><![CDATA[Trump's US drug prices order: what is Most Favoured Nation status?]]></title>
            <description><![CDATA[The president promises sweeping price cuts "almost immediately", but experts say these are unlikely.]]></description>
            <link>https://www.bbc.com/news/articles/c93l7k3x5dpo</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/c93l7k3x5dpo#2</guid>
            <pubDate>Mon, 12 May 2025 22:02:09 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/5218/live/875faf50-2f66-11f0-8ff1-59f5dcf8e9f5.jpg"/>
        </item>
        <item>
            <title><![CDATA[BBC team's tense encounter with sanctioned Israeli settler while filming in West Bank]]></title>
            <description><![CDATA[ Israeli settler Moshe Sharvit established a so-called 'outpost'  in occupied West Bank]]></description>
            <link>https://www.bbc.com/news/articles/c8d1j3v2y3mo</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/c8d1j3v2y3mo#2</guid>
            <pubDate>Sun, 11 May 2025 21:20:47 GMT</pubDate>
            <media:thumbnail width="240" height="134" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/b71a/live/22a75cf0-2eb4-11f0-8ff1-59f5dcf8e9f5.png"/>
        </item>
        <item>
            <title><![CDATA[The rare disease in a remote town where 'almost everyone is a cousin']]></title>
            <description><![CDATA[Many children in a Brazil town lost the ability to walk, so one scientist set about discovering why.]]></description>
            <link>https://www.bbc.com/news/articles/ce80d4yppl3o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/ce80d4yppl3o#2</guid>
            <pubDate>Sun, 11 May 2025 23:05:45 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/fac6/live/0a621d00-f45e-11ef-896e-d7e7fb1719a4.png"/>
        </item>
        <item>
            <title><![CDATA[How backchannels and US mediators pulled India and Pakistan back from the brink]]></title>
            <description><![CDATA[India and Pakistan agree to a full and immediate ceasefire after days of cross-border strikes.]]></description>
            <link>https://www.bbc.com/news/articles/clyn617xv4no</link>
            <guid isPermaLink="false">https://www.bbc.com/news/articles/clyn617xv4no#2</guid>
            <pubDate>Sun, 11 May 2025 06:08:13 GMT</pubDate>
            <media:thumbnail width="240" height="134" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/737f/live/4d4b47a0-2e3c-11f0-8ff1-59f5dcf8e9f5.jpg"/>
        </item>
        <item>
            <title><![CDATA[Watch: Abuse and blackmail - Cassie describes Diddy relationship at trial]]></title>
            <description><![CDATA[The prosecution wrapped up its questioning of Ventura, who will return the stand on Thursday for cross-examination by Combs' lawyers.]]></description>
            <link>https://www.bbc.com/news/videos/c04e74x6g21o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/videos/c04e74x6g21o#3</guid>
            <pubDate>Wed, 14 May 2025 23:52:55 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/9b74/live/abafab60-311b-11f0-96c3-cf669419a2b0.jpg"/>
        </item>
        <item>
            <title><![CDATA[Key looks from the Cannes Film Festival red carpet]]></title>
            <description><![CDATA[From Quentin Tarantino to Bella Hadid - celebrities hit the red carpet as the 78th Cannes Film Festival kicked off in the South of France.]]></description>
            <link>https://www.bbc.com/news/videos/cvgn69g3n1ro</link>
            <guid isPermaLink="false">https://www.bbc.com/news/videos/cvgn69g3n1ro#3</guid>
            <pubDate>Wed, 14 May 2025 02:50:01 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/e1d7/live/fa63be00-306b-11f0-96c3-cf669419a2b0.jpg"/>
        </item>
        <item>
            <title><![CDATA[Watch timelapse of Trump's marathon handshake session with Saudis]]></title>
            <description><![CDATA[Dozens of senior officials, military leaders and businessmen queued to greet the US president in Saudi Arabia.]]></description>
            <link>https://www.bbc.com/news/videos/c93y8835dzjo</link>
            <guid isPermaLink="false">https://www.bbc.com/news/videos/c93y8835dzjo#3</guid>
            <pubDate>Tue, 13 May 2025 14:10:28 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/037d/live/f7cd4c60-3002-11f0-8519-3b5a01ebe413.jpg"/>
        </item>
        <item>
            <title><![CDATA[Watch: Thermal camera captures moment volcano erupts]]></title>
            <description><![CDATA[The volcano on the Negros island spewed plumes of ash around 3km (1.8 miles) high into the air.]]></description>
            <link>https://www.bbc.com/news/videos/cp3nzdg6293o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/videos/cp3nzdg6293o#3</guid>
            <pubDate>Tue, 13 May 2025 08:36:14 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/b8af/live/eb054e40-2fcd-11f0-8f57-b7237f6a66e6.jpg"/>
        </item>
        <item>
            <title><![CDATA[Gaza parents desperate as children face starvation under Israeli blockade]]></title>
            <description><![CDATA[The BBC's Fergal Keane reports on the rise of malnutrition in Gaza's children.]]></description>
            <link>https://www.bbc.com/news/videos/cwyn9291yv0o</link>
            <guid isPermaLink="false">https://www.bbc.com/news/videos/cwyn9291yv0o#3</guid>
            <pubDate>Mon, 12 May 2025 22:11:38 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/b031/live/e1ab05d0-2f7b-11f0-b26b-ab62c890638b.jpg"/>
        </item>
        <item>
            <title><![CDATA[Watch: Crowds cheer as Pope holds first Sunday address]]></title>
            <description><![CDATA[Worshippers packed St Peter's Square as Pope Leo XIV called for an end to global conflicts.]]></description>
            <link>https://www.bbc.com/news/videos/c071n4339xzo</link>
            <guid isPermaLink="false">https://www.bbc.com/news/videos/c071n4339xzo#3</guid>
            <pubDate>Sun, 11 May 2025 11:54:56 GMT</pubDate>
            <media:thumbnail width="240" height="135" url="https://ichef.bbci.co.uk/ace/standard/240/cpsprodpb/8f4c/live/7f93edd0-2e5e-11f0-b26b-ab62c890638b.jpg"/>
        </item>
    </channel>
</rss>
"#;
        let entries = model.process_news_from_feed(news).await;
        println!("Entries: {:?}", entries);
        debug!("Entries: {:?}", entries);
        assert!(entries.is_ok());
    }
}

