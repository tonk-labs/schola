use serde::{Serialize, Deserialize};
use warp::Filter;
use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt, ToBigUint};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};
use std::io::{self, Write};

use crate::ot;


#[derive(Debug)]
pub struct InvalidRequest;

impl warp::reject::Reject for InvalidRequest {}

#[derive(Debug)]
pub struct InvalidState;

impl warp::reject::Reject for InvalidState {}

// Round 1 response from server
#[derive(Serialize, Deserialize)]
struct Round1Response {
    p: BigUint,
    g: BigUint,
    v: BigUint,
    beta: BigUint,
}

// Round 2 request from client
#[derive(Serialize, Deserialize)]
struct Round2Request {
    v: BigUint,
    beta: BigUint,
    u: BigUint,
}

// Round 2 response from server
#[derive(Serialize, Deserialize, Debug)]
struct Round2Response {
    v: BigUint,
    c_j: Vec<Vec<u8>>,
}

// Server state
struct ServerState {
    round: u8,
    round1_response: Round1Response,
}

// Server
pub async fn run_server() {
    let (p, g,v,beta)= ot::sender_round_1();
    let state = Arc::new(Mutex::new(ServerState {
        round: 1,
        round1_response: Round1Response {
            p: p,
            g: g,
            v: v,
            beta: beta,
        },
    }));

    let state = warp::any().map(move || state.clone());

    let routes = warp::post()
        .and(warp::path("ot"))
        .and(warp::body::json())
        .and(state)
        .and_then(handle_request);

    println!("Server running on http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_request(
    body: serde_json::Value,
    state: Arc<Mutex<ServerState>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut state = state.lock().await;

    match state.round {
        1 => {
            state.round = 2;
            Ok(warp::reply::json(&state.round1_response))
        }
        2 => {
            let request: Round2Request = serde_json::from_value(body).map_err(|_| warp::reject::custom(InvalidRequest))?;
            
            // Here you would typically process the request and generate a real response
            let (v, c_j) = ot::sender_round_2(request.v, request.beta, request.u);
            let response = Round2Response {
                v: v,
                c_j: c_j
            };

            state.round = 1; // Reset for next OT protocol run
            Ok(warp::reply::json(&response))
        }
        _ => Err(warp::reject::custom(InvalidState)),
    }
}

// Client
pub struct Client {
    id: String,
    server_url: String,
}

impl Client {
    pub fn new(id: String, server_url: String) -> Self {
        Self { id, server_url }
    }

    pub async fn run_ot_protocol(&self, i: u32) -> Result<(), Box<dyn std::error::Error>> {
        // Round 1
        let r1: Round1Response = reqwest::Client::new()
            .post(&format!("{}/ot", self.server_url))
            .json(&serde_json::json!({}))
            .send()
            .await?
            .json()
            .await?;

        let v = r1.v.clone();

        let (u, i, alpha) = ot::receiver_round_1(i, &r1.p, &r1.g, &v);

        // Process round1_response and prepare round2_request
        // This is a placeholder and should be implemented according to the OT protocol
        let round2_request = Round2Request {
            v: r1.v.clone(),
            beta: r1.beta,
            u: u, // This should be generated according to the protocol
        };

        // Round 2
        let r2: Round2Response = reqwest::Client::new()
            .post(&format!("{}/ot", self.server_url))
            .json(&round2_request)
            .send()
            .await?
            .json()
            .await?;

        let m_ascii = ot::receiver_round_2(&v, &alpha, &r1.p, i, &r2.c_j);
        
        println!("Decrypted message as ASCII: {}", m_ascii);
        // Process round2_response
        println!("Received round 2 response: {:?}", r2);

        Ok(())
    }

}