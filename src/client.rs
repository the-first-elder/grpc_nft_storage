use nft::nft_service_client::NftServiceClient;
use nft::{
    MetadataListResponse, MetadataRequest, MetadataResponse, MetadataUpdateRequest, TokenId,
};
use std::collections::HashMap;
use std::iter::Map;
use tonic::transport::Channel;
use tonic::Request;
pub mod nft {
    tonic::include_proto!("nft");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NftServiceClient::connect("http://[::1]:50051").await?;
    // save metadata
    let mut attributes: HashMap<String, String> = HashMap::new();
    attributes.insert("color".to_string(), "green".to_string());
    attributes.insert("size".to_string(), "small".to_string());

    let request: Request<MetadataRequest> = Request::new(MetadataRequest {
        name: "Updated NFT".to_string(),
        description: "This is an updated NFT.".to_string(),
        image_uri: "https://example.com/updated_image.jpg".to_string(),
        attributes: attributes,
    });
    save_metadata(&mut client, request).await?;
    println!("--------------------------------");

    // get metadata
    get_metadata(&mut client, 1.to_string()).await?;
    println!("--------------------------------");

    // update metadat

    let mut attributes: HashMap<String, String> = HashMap::new();
    attributes.insert("color".to_string(), "green".to_string());
    attributes.insert("size".to_string(), "small".to_string());

    let request = Request::new(MetadataUpdateRequest {
        id: "1".to_string(),
        name: "Updated NFT".to_string(),
        description: "This is an updated NFT.".to_string(),
        image_uri: "https://example.com/updated_image.jpg".to_string(),
        attributes: attributes,
    });
    update_metadata(&mut client, request).await?;
    println!("--------------------------------");

    // list all metadata
    list_all_metadata(&mut client).await?;
    println!("--------------------------------");
    // delete metadata
    delete_metadata(&mut client, 1.to_string()).await?;
    println!("--------------------------------");

    //  list all metadata
    list_all_metadata(&mut client).await?;

    Ok(())
}

async fn save_metadata(
    client: &mut NftServiceClient<Channel>,
    request: Request<MetadataRequest>,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.store_metadata(request).await?;
    println!("Nft metdata saved : {:?}", response);

    Ok(())
}

async fn get_metadata(
    client: &mut NftServiceClient<Channel>,
    id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::new(TokenId { id: id });
    let response = client.get_metadata(request).await?;
    println!("Nft metdata retrieved : {:?}", response);
    Ok(())
}

async fn update_metadata(
    client: &mut NftServiceClient<Channel>,
    request: Request<MetadataUpdateRequest>,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.update_metadata(request).await?;
    println!("metadat updated{:?}", response);
    Ok(())
}

async fn delete_metadata(
    client: &mut NftServiceClient<Channel>,
    id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::new(TokenId { id: id });
    let _response = client.delete_metadata(request).await?;
    println!("metadata deleted");
    Ok(())
}

async fn list_all_metadata(
    client: &mut NftServiceClient<Channel>,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::new(());
    let response = client.list_all_metadata(request).await?;
    println!("All metadata : {:?}", response);
    Ok(())
}
