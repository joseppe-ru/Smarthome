use warp::Filter;

#[tokio::main]
async fn main() {
	let hello = warp::path("hello-munke")
		.and(warp::path::param())
		.and(warp::header("PiSer-Modul"))
		.map(|param: String, agent: String| {
        	format!("Hello {}, whose agent is {}", param, agent)});

	warp::serve(hello)
		.run(([127,0,0,1],3030))
		.await;
}
