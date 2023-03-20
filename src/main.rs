mod tba_data;

use tba_data::TBAData;

use rouille::{Request, Response};
use serde_json;
use std::io::Read;

// This function is called in a thread for each incoming request, meaning I can
// use it from data reading, parsing, and interpreting, all the way to
// constructing a request for the Slack webhook, all without worrying
// about blocking threads for other requests
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

    // I'm still not super happy with this, as it is kind of mixing concerns.
    // I feel like there is a better option for early returning *or* assigning the variable
    // It does however complete my goal of a clean happy path
    let data: TBAData = match serde_json::from_reader(&buf[..]) {
        Ok(data) => data,
        Err(_) => return Response::text("Invalid TBA Notification").with_status_code(400),
    };

    println!("{data:?}");

    Response::text("")
}

fn main() {
    rouille::start_server("0.0.0.0:5369", webhook_handler);
}
