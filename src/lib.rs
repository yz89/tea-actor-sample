// Copyright 2015-2020 Capital One Services, LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and 
// limitations under the License.
#[macro_use]
extern crate log;
extern crate wascc_actor as actor;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

use actor::prelude::*;
const CUSTOM_OPERATION: &str = "hello";
const CAPABILITY_ID_TPM: &str = "tea:tpm";
const CAPABILITY_ID_LAYER1: &str = "tea:layer1";

actor_handlers!{ codec::http::OP_HANDLE_REQUEST => hello_world,
    codec::core::OP_HEALTH_REQUEST => health }

fn hello_world(payload: codec::http::Request) -> HandlerResult<codec::http::Response> {
    info!("Received request: {:?}", payload);
    if payload.path != "/favicon.ico" {
        logger::default().warn(&format!("Received an HTTP request: {:?}", payload))?;//in order to see this log output, please run the tea runtime using "RUST_LOG=warn cargo run" command
    }
    let res = match payload.path.as_str() {
        "/" => {
            let test_url_links: &str = " <html><head></head><body>\
             <p><a href='http://localhost:8081/tpm?cmd=get_properties'>\
             http://localhost:8081/get_properties\
             </a></p> <p>\
             <a href='http://localhost:8081/tpm?cmd=get_pcr'>\
             http://localhost:8081/tpm?cmd=get_pcr\
             </a></p>\
              </body>\
              </head>\
              </html> ";
            return Ok(codec::http::Response{status_code:200, status:String::from("OK"), header:std::collections::HashMap::new(), body:test_url_links.as_bytes().to_vec()})
        },
        "/tpm" => untyped::default().call(
            CAPABILITY_ID_TPM,
            CUSTOM_OPERATION,
            serialize(CustomMessage { command: String::from(payload.query_string)})?,
        )?,
        "/layer1" => untyped::default().call(
            CAPABILITY_ID_LAYER1,
            CUSTOM_OPERATION,
            serialize(CustomMessage { command: String::from(payload.query_string) })?,
        )?,
        _=> serialize(CustomReply { answer:  String::from("Default")})?,
    };

    let reply: CustomReply = deserialize(&res)?;

    let result = json!({ "calling actor got result": reply.answer });
    Ok(codec::http::Response::json(result, 200, "OK"))
}

fn health(_req: codec::core::HealthRequest) -> HandlerResult<()> {
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMessage {
    pub command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomReply {
    pub answer: String,
}