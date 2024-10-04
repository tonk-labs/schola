mod network;
mod ot;
mod config;

use std::env;
use config::Settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new()?;
    let server_url = settings.server.url;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} --server | --client", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "--server" => {
            println!("Starting server...");
            network::run_server().await;
        }
        "--client" => {
            println!("Starting client...");
            let client = network::Client::new(
                "client1".to_string(),
                server_url,
            );

            println!("Enter a number for the OT protocol:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let choice: u32 = input.trim().parse().expect("Please enter a valid number");

            match client.run_ot_protocol(choice).await {
                Ok(_) => println!("OT protocol completed successfully"),
                Err(e) => eprintln!("Error running OT protocol: {}", e),
            }
        }
        _ => {
            eprintln!("Invalid argument. Use --server or --client");
            std::process::exit(1);
        }
    }

    Ok(())
}

// fn main_OT() {
//     //shared values
//     let p = get_p();
//     let g = get_g();
//     // Sender round 1
//     let (p, g, v, beta) = sender_round_1();
//     let mut rng = OsRng;

//     // Receiver round 1
//     let (u, i, alpha) = receiver_round_1(&p, &g, &v);

//     // Sender round 2
//     let (v, c_j) = sender_round_2(v, beta, u);

//     // Receiver round 2
//     let m_ascii = receiver_round_2(&v, &alpha, &p, i, &c_j);
//     println!("Decrypted message as ASCII: {}", m_ascii);
// }
