use bb_grpc::update_service_client::UpdateServiceClient;
use bb_grpc::{UpdateRpcRequest, UpdateRpcResponse};

pub mod bb_grpc {
    tonic::include_proto!("bb_grpc");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

async fn request_updates(
    current_state: UpdateRpcRequest,
) -> Result<UpdateRpcResponse, Box<dyn std::error::Error>> {
    let mut client = UpdateServiceClient::connect("http://[::1]:50051").await?;

    match client.update_rpc(current_state).await {
        Ok(response) => Ok(response.into_inner()),
        Err(e) => Err(Box::new(e)),
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use bb_grpc::{
        UnitDirectionVector, UnitPosition, UnitState, UnitType, UpdateRpcRequest, UpdateStatus,
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
        processing_time_ns: u64,
    ) -> UpdateRpcRequest {
        let mut rng = thread_rng();
        UpdateRpcRequest {
            status: UpdateStatus::Processing as i32,
            update_id: rng.gen::<u32>(),
            units: generate_sample_units(unit_count),
            per_unit_proc_time_ns: processing_time_ns,
        }
    }

    struct Output {
        unit_count: usize,
        on_chip_processing_time_us: u64,
        round_trip_latency_us: u128,
    }

    impl fmt::Display for Output {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{:>11} |{:>21} |{:>13} |",
                self.unit_count, self.on_chip_processing_time_us, self.round_trip_latency_us
            )
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_node_performance() {
        let processing_time_ns = 100000;
        let team_sizes = (0..=200).step_by(10).collect::<Vec<usize>>();
        let mut results: Vec<Output> = vec![];
        for unit_count in team_sizes {
            let current_state = generate_update_state_request(unit_count, processing_time_ns);

            let start = std::time::Instant::now();
            let response = request_updates(current_state).await.unwrap();
            let round_trip_latency_us = start.elapsed().as_micros();

            results.push(Output {
                unit_count,
                on_chip_processing_time_us: response.single_pass_elapsed_time_us,
                round_trip_latency_us,
            });
        }
        println!("To simulate unit processing e.g. running A* etc on each unit");
        println!(
            "Apply constant processing time of {}ns per unit",
            processing_time_ns
        );
        println!(" Unit Count | Processing Time (us) | Latency (us) |");
        results.iter().for_each(|x| println!("{}", x));
    }
}
