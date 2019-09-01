use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/shutdown")]
fn shutdown() -> impl Responder {
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(2));
        std::process::Command::new("shutdown")
            .arg("now")
            .spawn()
            .expect("failed to start shutdown");
    });
    HttpResponse::Ok().body("Shutting down...")
}

#[get("/reboot")]
fn reboot() -> impl Responder {
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(2));
        std::process::Command::new("reboot")
            .spawn()
            .expect("failed to start reboot");
    });
    HttpResponse::Ok().body("Rebooting...")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("IP address wasn't provided. Please provide IP address");
    } else {
        HttpServer::new(|| App::new().service(shutdown).service(reboot))
            .bind(&args[1])
            .unwrap()
            .run()
            .unwrap();
    }
}
