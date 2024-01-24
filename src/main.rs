use game_server::update_game_state_client::UpdateGameStateClient;
use game_server::{
    UnitDirectionVector, UnitPosition, UnitState, UnitType, UpdateStateRequest, UpdateStatus,
    UpdatedStateResponse,
};
use rand::thread_rng;
use rand::Rng;

pub mod game_server {
    tonic::include_proto!("game_server");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = thread_rng();

    let sample_unit = UnitState {
        id: rng.gen::<u32>(),
        unit_type: UnitType::Mobile as i32,
        position: Some(UnitPosition {
            x: rng.gen::<f32>(),
            y: rng.gen::<f32>(),
            layer: rng.gen::<u32>(),
            direction: Some(UnitDirectionVector {
                x: rng.gen::<f32>(),
                y: rng.gen::<f32>(),
            }),
        }),
        tag: "Sample".into(),
    };

    let mut results: Vec<(usize, u32, u64, u128)> = vec![];

    let unit_counts = (0..=100).step_by(10).collect::<Vec<usize>>();
    let stress_multiple = (0..=10).step_by(1).collect::<Vec<u32>>();
    for step_count in unit_counts.iter() {
        for multiple in &stress_multiple {
            let current_state = UpdateStateRequest {
                status: UpdateStatus::Processing as i32,
                update_id: rng.gen::<u32>(),
                units: vec![sample_unit.clone(); *step_count],
                multiplier: *multiple,
            };

            let start = std::time::Instant::now();
            let response = request_updates(current_state).await?;
            let round_trip = start.elapsed();

            let on_chip_processing = response.single_pass_elapsed_time * (*multiple as u64);

            results.push((
                *step_count,
                *multiple,
                on_chip_processing,
                round_trip.as_micros(),
            ))
        }
    }

    print_output(results);
    // if response.metadata().contains_key("date") {
    //     println!(
    //         "Message timestamp: {:?}",
    //         response.metadata().get("date").unwrap()
    //     );
    // }
    // println!("Message response: {:?}", response.into_inner());
    // //println!("RESPONSE={:?}", response);

    Ok(())
}

async fn request_updates(
    current_state: UpdateStateRequest,
) -> Result<UpdatedStateResponse, Box<dyn std::error::Error>> {
    let mut client = UpdateGameStateClient::connect("http://[::1]:50051").await?;

    match client.update(current_state).await {
        Ok(response) => Ok(response.into_inner()),
        Err(e) => Err(Box::new(e)),
    }
}

fn print_output(results: Vec<(usize, u32, u64, u128)>) {
    println!("Unit count | Multiple | Processing Time (us) | Round Trip(us)");
    for x in results.iter() {
        println!("{:>10},{:>10},{:>22},{:>16},", x.0, x.1, x.2, x.3);
    }
}
