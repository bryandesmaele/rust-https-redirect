extern crate hyper;
extern crate iron;
#[macro_use] extern crate router;

use iron::modifiers::Redirect;
use iron::prelude::*;
use iron::status;
use iron::Url;

fn main() {
    let router = router!{
        health: get "/redirect_service_health" => health_handler,
        path_request: get "/**" => request_handler,
        root_request: get "/" => request_handler
    };

    println!("Listening on 0.0.0.0:9001");
    Iron::new(router).http("0.0.0.0:9001").unwrap();

    fn health_handler(_r: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok,"UP")))
    }

    fn request_handler(req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((
            status::MovedPermanently, 
            Redirect(
                Url::parse(
                    &format!("https://{}", req.url.host())
                ).unwrap()
            )
        )))
   }   
}
