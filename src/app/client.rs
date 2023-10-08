use messages::message_client::MessageClient;
use messages::MessageRequest;

pub mod messages {
    tonic::include_proto!("messages");
}

//main is for sending
pub async fn send_msg(
    username: String,
    msg: String,
    passw: String,
    ip: String,
    is_sync: bool,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut client = MessageClient::connect(format!("http://{}", ip)).await?;

    let request = tonic::Request::new(MessageRequest {
        message: msg,
        sent_by: username,
        is_sync: is_sync,
        password: passw,
    });

    let response = client.send_message(request).await?.into_inner().clone();

    let date = response.message_time;

    let message = response.message;

    Ok((message, date))
}
