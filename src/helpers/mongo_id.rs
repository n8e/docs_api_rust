use regex::Regex;
use rocket::request::FromParam;

#[derive(Clone)]
pub struct MongoId(String);

impl MongoId {
    pub fn to_string(&self) -> String {
        self.clone().0
    }
}

impl<'a> FromParam<'a> for MongoId {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<MongoId, Self::Error> {
        let re = Regex::new(r"^[a-f0-9]{24}$").unwrap();
        let m = re.is_match(param);

        if m == true {
            let p = MongoId(param.to_string());
            Ok(p)
        } else {
            Err(param)
        }
    }
}
