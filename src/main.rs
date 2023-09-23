use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Debug)]
struct GuestbookService;

pub mod guestbook {
    tonic::include_proto!("guestbook");
}

// why?? crate::guestbook::entries_server::Entries
use guestbook::guestbooks_server::{Guestbooks, GuestbooksServer};
use guestbook::{
    CreateGuestbook, CreateGuestbookResponse, DeleteGuestbookRequest, DeleteGuestbookResponse,
    GetGuestbookRequest, GetGuestbooksRequest, Guestbook, GuestbookResponse, GuestbooksResponse,
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
            start_date, end_date, host, image_path, entries
        } = req.into_inner();

        let reply = guestbook::Guestbook {
            id: 69,
            start_date: 1,
            end_date: 2,
            host: "Potter".to_owned(),
            image_path: "".to_owned(),
            entries: Vec::new(),
        };

        Ok(Response::new(CreateGuestbookResponse { guestbook: Some(reply) }))
    }

    async fn get(
        &self,
        req: Request<GetGuestbookRequest>,
    ) -> Result<Response<GuestbookResponse>, Status> {
        unimplemented!()
    }

    async fn get_many(
        &self,
        req: Request<GetGuestbooksRequest>,
    ) -> Result<Response<GuestbooksResponse>, Status> {
        unimplemented!()
    }

    async fn search(
        &self,
        req: Request<SearchGuestbookRequest>,
    ) -> Result<Response<GuestbooksResponse>, Status> {
        unimplemented!()
    }

    async fn update(
        &self,
        req: Request<UpdateGuestbookRequest>,
    ) -> Result<Response<UpdateGuestbookResponse>, Status> {
        unimplemented!()
    }

    async fn delete(
        &self,
        req: Request<DeleteGuestbookRequest>,
    ) -> Result<Response<DeleteGuestbookResponse>, Status> {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    let guest_book = GuestbookService {};

    let svc = GuestbooksServer::new(guest_book);

    println!("Server running on {}... in theory...", addr);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
