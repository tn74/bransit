use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod mta {
    use gtfs_structures;
    pub fn example () {
      // Gtfs::new will try to guess if you provide a path, a local zip file or a remote zip file.
      // You can also use Gtfs::from_path or Gtfs::from_url
      let gtfs = gtfs_structures::Gtfs::new("/Users/trishulnagenalli/Downloads/google_transit.zip").unwrap();
      println!("there are {} stops in the gtfs", gtfs.stops.len());

      // This structure is the easiest to use as the collections are `HashMap`,
      // thus you can access an object by its id.
      let route_1 = gtfs.routes.get("1").expect("no route 1");
      println!("{}: {:?}", route_1.short_name, route_1);
    }
    
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    mta::example();
    println!("HELLO");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
