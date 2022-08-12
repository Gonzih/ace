use anyhow::Result;
use serde::{Deserialize, Serialize};

pub(crate) fn deserialize<'a, T>(data: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let r = bincode::deserialize(data)?;

    Ok(r)
}

pub(crate) fn serialize<T: ?Sized>(v: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let r = bincode::serialize(v)?;

    Ok(r)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TestRequest {
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TestResponse {
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Request {
    Test(TestRequest),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Response {
    Test(TestResponse),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bincode_works() {
        let msg = Request::Test(TestRequest {
            data: "genrel".to_string(),
        });
        let encoded: Vec<u8> = serialize(&msg).unwrap();

        assert_eq!(encoded.len(), 18);

        let decoded: Request = deserialize(&encoded[..]).unwrap();
        assert_eq!(decoded, msg);
    }
}
