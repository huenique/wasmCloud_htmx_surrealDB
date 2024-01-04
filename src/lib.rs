wit_bindgen::generate!({
    world: "hello",
    exports: {
        "wasi:http/incoming-handler": HttpServer,
    },
});

use askama::Template;
use exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::*;

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

struct HttpServer;

impl Guest for HttpServer {
    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {
        let hello = HelloTemplate { name: "world" };
        let template = hello.render().unwrap();

        // println!("template: {}", template);

        // let response = OutgoingResponse::new(
        //     Fields::from_list(&[
        //         (":status".into(), "200".into()),
        //         ("content-type".into(), "text/html".into()),
        //     ])
        //     .unwrap(),
        // );
        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(200).unwrap();
        let response_body = response.body().unwrap();
        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(template.as_bytes())
            .unwrap();
        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
        ResponseOutparam::set(response_out, Ok(response));
    }
}
