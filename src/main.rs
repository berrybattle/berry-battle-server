use game_server::update_game_state_client::UpdateGameStateClient;
use game_server::{UpdateStateRequest, UpdateStatus};
use prost_types::Timestamp;

pub mod game_server {
    tonic::include_proto!("game_server");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UpdateGameStateClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(UpdateStateRequest {
        // name: "Tonic".into(),
        status: UpdateStatus::Finished as i32,
        update_id: "1234".into(),
        timestamp: Some(Timestamp::default()),
        data: "Blob".into(),
    });

    let response = client.update(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
