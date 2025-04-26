use axum::{Router, extract::State, routing::get};
use std::env;
use std::net::SocketAddrV4;
use tokio::net::TcpListener;
use wakey::WolPacket;

#[derive(Clone)]
struct Config {
    dest_ip: String,
    src_ip: String,
    mac: String,
}

async fn wake_on_lan(State(config): State<Config>) -> Result<String, String> {
    let mac = config.mac;
    let to_ip = config.dest_ip;
    let from_ip = config.src_ip;
    let packet =
        WolPacket::from_string(&mac, ':').map_err(|e| format!("Invalid MAC address: {:?}", e))?;
    let to_ip = SocketAddrV4::new(
        to_ip
            .parse::<std::net::Ipv4Addr>()
            .map_err(|_| "Invalid IP address".to_string())?,
        9,
    );
    let from_ip = SocketAddrV4::new(
        from_ip
            .parse::<std::net::Ipv4Addr>()
            .map_err(|_| "Invalid IP address".to_string())?,
        0,
    );
    println!("Sending magic packet to {} from {}", to_ip, from_ip);
    packet
        .send_magic_to(from_ip, to_ip)
        .map_err(|e| format!("Failed to send magic packet: {:?}", e))?;
    Ok("OK".into())
}

#[tokio::main]
async fn main() {
    let dest_ip = env::var("TARGET_IP_ADDRESS").unwrap();
    let mac = env::var("TARGET_MAC_ADDRESS").unwrap();
    let src_ip = env::var("SOURCE_IP_ADDRESS").unwrap();
    let conf = Config {
        dest_ip,
        mac,
        src_ip,
    };
    println!("IP: {}, MAC: {}", conf.dest_ip, conf.mac);
    let router = Router::new().route("/", get(wake_on_lan)).with_state(conf);

    let listener = TcpListener::bind("127.0.0.1:5000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
