mod tba_data;

use tba_data::TBAData;

use rouille::{Request, Response};
use serde_json;
use std::io::Read;

fn webhook_handler<'a>(req: &'a Request) -> Response {
    // TODO:: This need to return 400 bad requests for all;
    //          - non TBA ips
    //          - non POST requests
    //          - no Body post requests
    //          - prolly something else
    let mut data = req.data().expect("shouldve had data here...");

    let mut buf = Vec::new();
    if let Err(_) = data.read_to_end(&mut buf) {
        return Response::text("Failed to read body").with_status_code(400);
    }

    if let Ok(data) = serde_json::from_reader::<_, TBAData>(&buf[..]) {
        println!("{data:?}");

        Response::text("")
    } else {
        Response::text("Body not valid TBA Notification").with_status_code(400)
    }
}

fn main() {
    rouille::start_server("0.0.0.0:5369", webhook_handler);
}
