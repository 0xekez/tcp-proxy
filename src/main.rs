use clap::{App, Arg};
use tokio::io;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::select;

#[tokio::main]
async fn main() -> io::Result<()> {
    let matches = App::new("proxy")
        .version("0.1")
        .author("Zeke M. <zekemedley@gmail.com>")
        .about("A simple tcp proxy")
        .arg(
            Arg::with_name("eyeball")
                .short("e")
                .long("eyeball")
                .value_name("ADDRESS")
                .help("e address of the eyeball that we will be proxying traffic for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("origin")
                .short("o")
                .long("origin")
                .value_name("ADDRESS")
                .help("The address of the origin that we will be proxying traffic for")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let eyeball = matches.value_of("eyeball").unwrap();
    let origin = matches.value_of("origin").unwrap();

    // Listen for connections from the eyeball and forward to the
    // origin.

    let listener = TcpListener::bind(eyeball).await?;
    loop {
        let (eyeball, _) = listener.accept().await?;
        let origin = TcpStream::connect(origin).await?;

        let (mut eread, mut ewrite) = eyeball.into_split();
        let (mut oread, mut owrite) = origin.into_split();

        let e2o = tokio::spawn(async move { io::copy(&mut eread, &mut owrite).await });
        let o2e = tokio::spawn(async move { io::copy(&mut oread, &mut ewrite).await });

        select! {
                _ = e2o => println!("e2o done"),
                _ = o2e => println!("o2e done"),

        }
    }
}
