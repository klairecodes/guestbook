# Useful commands
## to see what services are exposed on a grpc server
`grpcurl -plaintext -import-path ./proto -proto guestbook.proto '[::1]:10000' list`
    - `-plaintext` is to avoid the TLS handshake
    - reflection API must be supported in order for `list` to work
        - `tonic_reflection`?
        - TODO
            - https://docs.rs/tonic/latest/tonic/macro.include_file_descriptor_set.html
            - https://github.com/hyperium/tonic/blob/master/examples/src/reflection/server.rs

- grpcurl docker container I hardly know her
    - https://github.com/fullstorydev/grpcurl#docker
- command to run grpcurl under podman container (Arch aur package broken :( )
    - `podman run --network="host" -v /home/klaire/Dropbox/code/web-projects/guestbook/proto:/protos/ fullstorydev/grpcurl -import-path /protos -plaintext -proto /protos/guestbook.proto localhost:10000 guestbook.Guestbooks/get`

## SQLx
- use `cargo add sqlx --features runtime-async-std-native-tls,chrono,postgres,macros`
    - TLSsssssssssssssss

## Quickest PostgreSQL/Podman setup of all time:
https://medium.com/@pawanpg0963/run-postgresql-with-podman-as-docker-container-86ad392349d1


# Structure ####################################################################

Backend:
Rust server listens, main -> db.rs -queries-> PostgreSQL

Frontend:
React/TS frontend calls, serves from -> Rust server

## Scoped config example:
https://github.com/actix/actix-web/issues/1928#issuecomment-766613881
```rust
use actix_web::{web, App, HttpResponse, HttpServer};

// this function could be located in a different module
fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok().body("test")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

// this function could be located in a different module
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| HttpResponse::Ok().body("app")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    )
    .service(web::scope("/api").configure(scoped_config))
    .route("/", web::get().to(|| HttpResponse::Ok().body("/")));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```
