use clap::{App, Arg};
use tokio::io;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::select;

async fn proxy(client: &str, server: &str) -> io::Result<()> {
    let listener = TcpListener::bind(client).await?;
    loop {
        let (client, _) = listener.accept().await?;
        let server = TcpStream::connect(server).await?;

        let (mut eread, mut ewrite) = client.into_split();
        let (mut oread, mut owrite) = server.into_split();

        let e2o = tokio::spawn(async move { io::copy(&mut eread, &mut owrite).await });
        let o2e = tokio::spawn(async move { io::copy(&mut oread, &mut ewrite).await });

        // let e2o = io::copy(&mut eread, &mut owrite);
        // let o2e = io::copy(&mut oread, &mut ewrite);

        select! {
                _ = e2o => println!("c2s done"),
                _ = o2e => println!("s2c done"),

        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let matches = App::new("proxy")
        .version("0.1")
        .author("Zeke M. <zekemedley@gmail.com>")
        .about("A simple tcp proxy")
        .arg(
            Arg::with_name("client")
                .short("c")
                .long("client")
                .value_name("ADDRESS")
                .help("The address of the client that we will be proxying traffic for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("server")
                .short("s")
                .long("server")
                .value_name("ADDRESS")
                .help("The address of the server that we will be proxying traffic for")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let client = matches.value_of("client").unwrap();
    let server = matches.value_of("server").unwrap();

    proxy(client, server).await
}
