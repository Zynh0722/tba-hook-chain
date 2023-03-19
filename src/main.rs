use rouille::Request;
use rouille::Response;
use std::io::Read;

fn main() {
    rouille::start_server("0.0.0.0:5369", move |req| {
        let mut data = req.data().expect("shouldve had data here...");

        let mut buf = Vec::new();
        match data.read_to_end(&mut buf) {
            Ok(_) => (),
            Err(_) => return Response::text("Failed to read body")
        };

        if let Ok(str) = String::from_utf8(buf) {
            println!("{}\n{}", req.url(), str);
        }

        Response::text("")
    })
}

