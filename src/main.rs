use tonic::{Request, Response, Status};

#[derive(Debug)]
struct GuestbookService;

pub mod guestbook {
    tonic::include_proto!("guestbook");
}

// why?? crate::guestbook::entries_server::Entries
use guestbook::guestbooks_server::{Guestbooks, GuestbooksServer};
use guestbook::{Guestbook, GetGuestbookRequest, GetGuestbooksRequest, GuestbookResponse, CreateGuestbook, CreateGuestbookResponse, GuestbooksResponse, SearchGuestbookRequest, UpdateGuestbookRequest, UpdateGuestbookResponse, DeleteGuestbookRequest, DeleteGuestbookResponse};

#[tonic::async_trait]
impl Guestbooks for GuestbookService {
    async fn create(&self, req: Request<CreateGuestbook>) -> Result<Response<CreateGuestbookResponse>, Status> {
        unimplemented!()
    }

    async fn get(&self, req: Request<GetGuestbookRequest>) -> Result<Response<GuestbookResponse>, Status> {
        unimplemented!()
    }

    async fn get_many(&self, req: Request<GetGuestbooksRequest>) -> Result<Response<GuestbooksResponse>, Status> {
        unimplemented!()
    }

    async fn search(&self, req: Request<SearchGuestbookRequest>) -> Result<Response<GuestbooksResponse>, Status> {
        unimplemented!()
    }

    async fn update(&self, req: Request<UpdateGuestbookRequest>) -> Result<Response<UpdateGuestbookResponse>, Status> {
        unimplemented!()
    }

    async fn delete(&self, req: Request<DeleteGuestbookRequest>) -> Result<Response<DeleteGuestbookResponse>, Status> {
        unimplemented!()
    }
}

fn main() {
    println!("Hello, world!");
}
