mod db;

use tokio::runtime;
use tonic::{Request, Response, Status};

#[derive(Debug)]
struct GuestbookService;

// TODO: reflector API support
pub mod guestbook {
    tonic::include_proto!("guestbook");
}

// why?? crate::guestbook::entries_server::Entries
use guestbook::guestbooks_server::Guestbooks;
use guestbook::{
    CreateGuestbook, CreateGuestbookResponse, DeleteGuestbookRequest, DeleteGuestbookResponse,
    GetGuestbookRequest, GetGuestbooksRequest, GuestbookResponse, GuestbooksResponse,
    SearchGuestbookRequest, UpdateGuestbookRequest, UpdateGuestbookResponse,
};

#[tonic::async_trait]
impl Guestbooks for GuestbookService {
    async fn create(
        &self,
        req: Request<CreateGuestbook>,
    ) -> Result<Response<CreateGuestbookResponse>, Status> {
        println!("Got a request {:?}.", req);
        let CreateGuestbook {
            start_date: _,
            end_date: _,
            host: _,
            image_path: _,
            entries: _,
        } = req.into_inner();

        let reply = guestbook::Guestbook {
            id: 69,
            start_date: 1,
            end_date: 2,
            host: "Potter".to_owned(),
            image_path: "".to_owned(),
            entries: Vec::new(),
        };

        //sqlx::query(
        //"
        //INSERT INTO
        //",
        //)
        //.execute()
        //.await?;

        Ok(Response::new(CreateGuestbookResponse {
            guestbook: Some(reply),
        }))
    }

    async fn get(
        &self,
        _req: Request<GetGuestbookRequest>,
    ) -> Result<Response<GuestbookResponse>, Status> {
        let _get_guestbook_request = {};
        let result = guestbook::Guestbook {
            id: 69,
            start_date: 1,
            end_date: 2,
            host: "Potter".to_owned(),
            image_path: "".to_owned(),
            entries: Vec::new(),
        };

        Ok(Response::new(GuestbookResponse {
            guestbook: Some(result.into()),
        }))
    }

    async fn get_many(
        &self,
        _req: Request<GetGuestbooksRequest>,
    ) -> Result<Response<GuestbooksResponse>, Status> {
        unimplemented!()
    }

    async fn search(
        &self,
        _req: Request<SearchGuestbookRequest>,
    ) -> Result<Response<GuestbooksResponse>, Status> {
        unimplemented!()
    }

    async fn update(
        &self,
        _req: Request<UpdateGuestbookRequest>,
    ) -> Result<Response<UpdateGuestbookResponse>, Status> {
        unimplemented!()
    }

    async fn delete(
        &self,
        _req: Request<DeleteGuestbookRequest>,
    ) -> Result<Response<DeleteGuestbookResponse>, Status> {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let addr = "[::1]:10000".parse().unwrap();

    //let guest_book = GuestbookService {};

    //let svc = GuestbooksServer::new(guest_book);

    //println!("Server running on {}... in theory...", addr);
    //Server::builder().add_service(svc).serve(addr).await?;

    //let rt = runtime::Runtime::enter(&self);
    let rt = runtime::Runtime::new().unwrap();
    let _ = rt.enter();

    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS guestbook (
    uuid integer UNIQUE NOT NULL PRIMARY KEY,
    creation_time TIMESTAMP,
    start_date DATE,
    end_date DATE,
    host varchar,
    image_path varchar,
    entries varchar[]
    );
    ",
    )
    .execute(&db::APP_STATE.db_pool)
    .await?;

    sqlx::query(
        "INSERT INTO guestbook VALUES (
        '2024', '2024-01-01', '2024-01-01', 'Potter', 's3.dummy/', []
        );
        ",
    )
    .execute(&db::APP_STATE.db_pool)
    .await?;

    sqlx::query(
        "
        DROP TABLE guestbook;
        ",
    )
    .execute(&db::APP_STATE.db_pool)
    .await?;

    Ok(())
}
