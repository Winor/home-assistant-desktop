use hass_rs::client::HassClient;

use async_tungstenite::{
    stream::Stream,
    tokio::{connect_async, TokioAdapter},
    tungstenite::{Error, Message},
    WebSocketStream,
};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::sync::{mpsc, mpsc::Receiver, mpsc::Sender};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpStream,
};

use tauri::Url;

async fn ws_incoming_messages(
    mut stream: SplitStream<
        WebSocketStream<
            Stream<TokioAdapter<TcpStream>, TokioAdapter<impl AsyncRead + AsyncWrite + Unpin>>,
        >,
    >,
    to_user: Sender<Result<Message, Error>>,
) {
    loop {
        while let Some(message) = stream.next().await {
            let _ = to_user.send(message).await;
        }
    }
}

async fn ws_outgoing_messages(
    mut sink: SplitSink<
        WebSocketStream<
            Stream<TokioAdapter<TcpStream>, TokioAdapter<impl AsyncRead + AsyncWrite + Unpin>>,
        >,
        Message,
    >,
    mut from_user: Receiver<Message>,
) {
    loop {
        match from_user.recv().await {
            Some(msg) => sink.send(msg).await.expect("Failed to send message"),
            None => continue,
        }
    }
}

fn is_wss(url: &mut Url) {
    if url.scheme() == "https" {
        let _ = url.set_scheme("wss");
        return;
    }
    let _ = url.set_scheme("ws");
}

pub async fn create_client(url: Url, token: String) -> Result<HassClient, anyhow::Error> {
    let mut url = url;
    is_wss(&mut url);
    url.set_path("/api/websocket");

    println!("Connecting to - {}", url);
    let (wsclient, _) = connect_async(url).await?;
    let (sink, stream) = wsclient.split();

    //Channels to recieve the Client Command and send it over to Websocket server
    let (to_gateway, from_user) = mpsc::channel::<Message>(20);
    //Channels to receive the Response from the Websocket server and send it over to Client
    let (to_user, from_gateway) = mpsc::channel::<Result<Message, Error>>(20);

    // Handle incoming messages in a separate task
    tokio::spawn(ws_incoming_messages(stream, to_user));

    // Read from command line and send messages
    tokio::spawn(ws_outgoing_messages(sink, from_user));

    let mut client = HassClient::new(to_gateway, from_gateway);

    let _ = client.auth_with_longlivedtoken(&token).await?;

    println!("WebSocket connection and authethication works\n");

    // let _ = tokio::try_join!(read_handle, write_handle);

    Ok(client)
}
