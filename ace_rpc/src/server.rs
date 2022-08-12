use crate::messages::{deserialize, serialize, Request, Response, TestRequest, TestResponse};
use crate::rpc::connect;
use crate::topics;
use anyhow::Result;
use nats::asynk::Connection;

pub struct ARPCServer {
    nc: Connection,
}

impl ARPCServer {
    pub async fn connect() -> Result<Self> {
        let nc = connect().await?;

        Ok(Self { nc })
    }

    pub async fn run(&self) -> Result<()> {
        let sub = self.nc.queue_subscribe(topics::RPC, "backend").await?;

        while let Some(msg) = sub.next().await {
            let message: Request = deserialize(&msg.data)?;
            let resp: Response = match message {
                Request::Test(req) => Response::Test(self.test_handler(req)?),
            };
            let data = serialize(&resp)?;
            msg.respond(data).await?;
        }

        Ok(())
    }

    fn test_handler(&self, msg: TestRequest) -> Result<TestResponse> {
        Ok(TestResponse {
            data: format!("Response for {}", msg.data),
        })
    }
}
