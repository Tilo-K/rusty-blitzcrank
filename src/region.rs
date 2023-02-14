static VALID_REGIONS: &[&str] = &[
    "BR1", "EUN1", "EUW1", "JP1", "KR", "LA1", "LA2", "NA1", "OC1", "PH2", "RU", "SG2", "TH2",
    "TR1", "TW2", "VN2", "AMERICAS", "ASIA", "EUROPE",
];
#[derive(Debug)]
pub enum RegionError {
    InvalidRegionString,
}

pub struct Region {
    reg_str: String,
}

impl Region {
    pub fn from_str(reg_str: String) -> Result<Region, RegionError> {
        let is_valid = VALID_REGIONS
            .iter()
            .any(|reg| reg.eq_ignore_ascii_case(&reg_str));

        if !is_valid {
            return Err(RegionError::InvalidRegionString);
        }

        Ok(Region {
            reg_str: reg_str.to_ascii_uppercase(),
        })
    }

    pub fn url(&self) -> String {
        return format!(
            "https://{}.api.riotgames.com/",
            self.reg_str.to_ascii_lowercase()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::Region;

    #[test]
    fn create_valid_region() {
        let reg = Region::from_str("EUW1".to_owned()).unwrap();

        assert!(reg.url().eq("https://euw1.api.riotgames.com/"))
    }

    #[test]
    fn create_invalid_region() {
        let res = Region::from_str("CRINGE".to_owned());
        assert!(res.is_err())
    }
}