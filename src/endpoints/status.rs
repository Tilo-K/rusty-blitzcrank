use crate::api_key::*;
use crate::dispatcher::*;
use crate::region::*;
use crate::types::*;

pub fn get_platform_data(
    region: &Region,
    api_key: &mut ApiKey,
    wait_for_rate_limit: bool,
) -> Result<PlatformData, BlitzError> {
    let mut url = region.url();
    url = format!("{}lol/status/v4/platform-data", &url);

    let res = dispatcher::get(url, api_key, wait_for_rate_limit, region.get_endpoint())?;

    let platform_data: PlatformData =
        serde_json::from_str(&res).map_err(|_| BlitzError::BadJson)?;

    Ok(platform_data)
}
