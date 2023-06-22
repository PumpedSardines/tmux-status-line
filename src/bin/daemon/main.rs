#[macro_use]
extern crate dotenv_codegen;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tiny_http::{Response, Server};

#[derive(Clone)]
struct Context {
    harvest_loading: bool,
    harvest_error: bool,
    harvest_data: Option<harvest::api::RunningTimerInfo>,
}

const HARVEST_USERNAME: &str = dotenv!("HARVEST_USERNAME");
const HARVEST_PASSWORD: &str = dotenv!("HARVEST_PASSWORD");

fn main() {
    let server = Server::http("0.0.0.0:57192").unwrap();

    let api = harvest::api::Api::new(HARVEST_USERNAME, HARVEST_PASSWORD);

    let ctx = Arc::new(Mutex::new(Context {
        harvest_loading: true,
        harvest_error: false,
        harvest_data: None,
    }));
    let thread_counter = Arc::clone(&ctx);

    thread::spawn(move || loop {
        #[cfg(feature = "harvest")]
        {
            let mut ctx = thread_counter.lock().unwrap();

            match api.running_timer() {
                Err(_) => {
                    (*ctx).harvest_error = true;
                    (*ctx).harvest_loading = false;
                }
                Ok(data) => {
                    (*ctx).harvest_error = false;
                    (*ctx).harvest_loading = false;
                    (*ctx).harvest_data = data;
                }
            }
        }
        thread::sleep(Duration::from_millis(10_000));
    });

    for request in server.incoming_requests() {
        match request.url() {
            #[cfg(feature = "harvest")]
            "/harvest" => {
                let ctx = Arc::clone(&ctx);
                let ctx = ctx.lock().unwrap();

                let harvest_text = match ctx.harvest_error {
                    true => String::from("Something went wrong"),
                    false => match ctx.harvest_loading {
                        true => String::from("Loading..."),
                        false => serde_json::to_string(&ctx.harvest_data.clone())
                            .expect("This data strcuture can't fail it's serialization"),
                    },
                };
                let response = Response::from_string(harvest_text);

                #[allow(unused_must_use)]
                {
                    request.respond(response);
                }
            }
            _ => {}
        }
    }
}
