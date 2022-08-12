use crate::messages::{deserialize, serialize, Request, Response, TestRequest, TestResponse};
use crate::rpc::connect;
use crate::topics;
use anyhow::anyhow;
use anyhow::Result;
use nats::asynk::Connection;

pub struct ARPCClient {
    nc: Connection,
}

impl ARPCClient {
    pub async fn connect() -> Result<Self> {
        let nc = connect().await?;

        Ok(Self { nc })
    }

    async fn call(&self, req: Request) -> Result<Response> {
        let bytes = serialize(&req)?;
        let resp = self.nc.request(topics::RPC, bytes).await?;
        let resp: Response = deserialize(&resp.data[..])?;

        Ok(resp)
    }

    pub async fn test(&self, data: String) -> Result<TestResponse> {
        let msg = Request::Test(TestRequest { data });
        let resp = self.call(msg).await?;

        match resp {
            Response::Test(resp) => Ok(resp),
            _ => Err(anyhow!("Wrong response type!")),
        }
    }
}
