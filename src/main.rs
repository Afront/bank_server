use iron::{Iron, Request, Response, IronResult, AfterMiddleware, Chain, status};
use iron::error::{IronError};
use logger::Logger;
use router::{Router, NoRoute};

struct Custom404;

impl AfterMiddleware for Custom404 {
	fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
		println!("Hitting custom 404 middleware");

		if err.error.is::<NoRoute>() {
			Ok(Response::with((status::NotFound, "Custom 404 response")))
		} else {
			Err(err)
		}
	}
}

fn main() {
	let mut router = Router::new();
	router.get("/", handler, "example");

	env_logger::init();
	let (logger_before, logger_after) = Logger::new(None);

	let mut chain = Chain::new(router);

	chain.link_before(logger_before);

	chain.link_after(Custom404);
	chain.link_after(logger_after);

	match Iron::new(chain).http("127.0.0.1:3000") {
		Result::Ok(listening) => println!("{:?}", listening),
		Result::Err(err) => panic!("{:?}", err),
	}
}

fn handler(_: &mut Request) -> IronResult<Response> {
	Ok(Response::with((status::Ok, "Handling response")))
}