use nft::nft_service_server::{NftService, NftServiceServer};
use nft::{
    MetadataListResponse, MetadataRequest, MetadataResponse, MetadataUpdateRequest, TokenId,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tonic::{transport::Server, Request, Response, Status};
pub mod nft {
    tonic::include_proto!("nft");
}

#[derive(Debug, Default)]
struct MyNftService {
    data: Arc<Mutex<HashMap<String, MetadataResponse>>>,
}

#[tonic::async_trait]
impl NftService for MyNftService {
    async fn store_metadata(
        &self,
        metadata: Request<MetadataRequest>,
    ) -> Result<Response<MetadataResponse>, Status> {
        let data = metadata.into_inner();
        let mut db = self.data.lock().unwrap();
        let token_id = format!("{}", db.len() + 1);
        let response = MetadataResponse {
            name: data.name,
            description: data.description,
            image_uri: data.image_uri,
            attributes: data.attributes,
        };
        db.insert(token_id, response.clone());
        Ok(Response::new(response))
    }

    async fn get_metadata(
        &self,
        metadata: Request<TokenId>,
    ) -> Result<Response<MetadataResponse>, Status> {
        let data = metadata.into_inner();
        let db = self.data.lock().unwrap();
        match db.get(&data.id) {
            Some(value) => Ok(Response::new(value.clone())),
            None => Err(Status::not_found(format!(
                "No metadata found for token_id {}",
                data.id
            ))),
        }
    }

    async fn update_metadata(
        &self,
        metadata: Request<MetadataUpdateRequest>,
    ) -> Result<Response<MetadataResponse>, Status> {
        let data = metadata.into_inner();
        let mut db = self.data.lock().unwrap();
        match db.get(&data.id) {
            Some(value) => {
                let mut value = value.clone();
                value.name = data.name;
                value.description = data.description;
                value.image_uri = data.image_uri;
                value.attributes = data.attributes;
                db.insert(data.id, value.clone());
                Ok(Response::new(value))
            }
            None => Err(Status::not_found(format!(
                "No metadata found for {}",
                &data.id
            ))),
        }
    }
    async fn delete_metadata(&self, metadata: Request<TokenId>) -> Result<Response<()>, Status> {
        let data = metadata.into_inner();
        let mut db = self.data.lock().unwrap();
        match db.remove(&data.id) {
            Some(_) => Ok(Response::new(())),
            None => Err(Status::not_found(format!(
                "No metadata found for {}",
                &data.id
            ))),
        }
    }

    async fn list_all_metadata(
        &self,
        _request: Request<()>,
    ) -> Result<Response<MetadataListResponse>, Status> {
        let db = self.data.lock().unwrap();
        let response = MetadataListResponse {
            metadata: db.values().cloned().collect(),
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = MyNftService::default();
    println!("Starting gRPC server on {}", addr);
    Server::builder()
        .add_service(NftServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
