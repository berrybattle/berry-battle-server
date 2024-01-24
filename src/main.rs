use game_server::update_game_state_client::UpdateGameStateClient;
use game_server::{UpdateStateRequest, UpdatedStateResponse};

pub mod game_server {
    tonic::include_proto!("game_server");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use game_server::{
        UnitDirectionVector, UnitPosition, UnitState, UnitType, UpdateStateRequest, UpdateStatus,
    };
    use rand::thread_rng;
    use rand::Rng;
    use std::fmt;

    fn generate_sample_units(count: usize) -> Vec<UnitState> {
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
        vec![sample_unit.clone(); count]
    }

    fn generate_update_state_request(
        unit_count: usize,
        stress_multiple: u32,
    ) -> UpdateStateRequest {
        let mut rng = thread_rng();
        UpdateStateRequest {
            status: UpdateStatus::Processing as i32,
            update_id: rng.gen::<u32>(),
            units: generate_sample_units(unit_count),
            multiplier: stress_multiple,
        }
    }

    struct Results {
        step_count: usize,
        multiple: u32,
        on_chip_processing_time_us: u64,
        round_trip_latency_us: u128,
    }

    impl fmt::Display for Results {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{:>21} |{:>21} |{:>21} |{:>21} |",
                self.step_count,
                self.multiple,
                self.on_chip_processing_time_us,
                self.round_trip_latency_us
            )
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_node_performance() {
        let unit_counts = (0..=200).step_by(10).collect::<Vec<usize>>();
        let stress_ratings = (0..=20).step_by(1).collect::<Vec<u32>>();
        println!("      Step Count      |    Stress Multiple   | Processing Time (us) |      Latency (us)    |");
        for unit in unit_counts.iter() {
            for stress_multiple in stress_ratings.iter() {
                let current_state = generate_update_state_request(*unit, *stress_multiple);

                let start = std::time::Instant::now();
                let response = request_updates(current_state).await.unwrap();
                let round_trip_latency_us = start.elapsed().as_micros();

                let on_chip_processing_time_us =
                    response.single_pass_elapsed_time_us * (*stress_multiple as u64);

                println!(
                    "{}",
                    Results {
                        step_count: *unit,
                        multiple: *stress_multiple,
                        on_chip_processing_time_us,
                        round_trip_latency_us
                    }
                );
            }
        }
    }
}
