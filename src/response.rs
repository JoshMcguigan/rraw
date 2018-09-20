use RRAWResult;
use error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    json: ResponseInternal,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ResponseInternal {
    RateLimit {
        ratelimit: f32,
        errors: Vec<Vec<String>>,
    },
    Success {
        errors: Vec<Vec<String>>,
        data: ResponseData,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseData {
    url: String,
    drafts_count: u8,
    id: String,
    name: String,
}

impl From<Response> for RRAWResult<ResponseData> {
    fn from(response: Response) -> Self {
        match response.json {
            ResponseInternal::RateLimit { ratelimit, .. } => Err(Error::RateLimit(ratelimit)),
            ResponseInternal::Success { data, .. } => Ok(data),
        }
    }
}

// TODO this has only been tested on the submit route, determine if this can be used other places

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn rate_limit() {
        let response_raw = r#"
        {
            "json": {
                "ratelimit": 424.223181,
                "errors": [
                    [
                        "RATELIMIT",
                        "you are doing that too much. try again in 7 minutes.",
                        "ratelimit"
                    ]
                ]
            }
        }"#;

        let response : Response = serde_json::from_str(response_raw).unwrap();
        let result : RRAWResult<ResponseData> = response.into();

        assert_matches!(result.err().unwrap(), Error::RateLimit(_ratelimit));
    }

    #[test]
    fn success() {
        let response_raw = r#"
        {
            "json": {
                "errors": [],
                "data": {
                    "url": "https://www.reddit.com/r/test/comments/9h4p82/testing_rraw_1537360057/",
                    "drafts_count": 0,
                    "id": "9h4p82",
                    "name": "t3_9h4p82"
                }
            }
        }"#;

        let response : Response = serde_json::from_str(response_raw).unwrap();
        let result : RRAWResult<ResponseData> = response.into();
        let data = result.unwrap();

        assert_eq!("t3_9h4p82", data.name);
    }
}
