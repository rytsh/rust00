use actix_web::{App, HttpResponse, HttpServer, web};
use serde::Deserialize;

use rust00::gcd;

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Starting server at http://localhost:3000");
    server
        .bind("127.0.0.1:3000")
        .expect("Can not bind to port 3000")
        .run()
        .await
        .expect("Failed to run server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
                <input type="text" name="n" />
                <input type="text" name="m" />
                <button type="submit">Compute GCD</button>
            </form>
        "#,
    )
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let d = gcd(form.n, form.m);
    let response = format!(
        "The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
        form.n, form.m, d
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}
