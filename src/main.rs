mod tba_data;

use tba_data::TBAData;

use rouille::Request;
use rouille::Response;
use std::io::Read;
use serde_json;

fn main() {
    rouille::start_server("0.0.0.0:5369", move |req| {
        let mut data = req.data().expect("shouldve had data here...");

        let mut buf = Vec::new();
        match data.read_to_end(&mut buf) {
            Ok(_) => (),
            Err(_) => return Response::text("Failed to read body")
        };
        
        let data: TBAData = serde_json::from_reader(&buf[..]).expect("failed data parsing");

        println!("{:?}", data);

        Response::text("")
    })
}

