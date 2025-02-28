# gRPC NFT Metadata Service

## 🚀 Overview
This is a gRPC-based NFT metadata service that allows clients to store, retrieve, update, and delete NFT metadata. The service is implemented in Rust using `tonic` and `prost`.

## 📦 Features
- Store NFT metadata
- Retrieve metadata by token ID
- Update metadata
- Delete metadata
- List all stored metadata

## 🛠️ Setup
### **1. Clone the Repository**
```sh
git clone https://github.com/the-first-elder/grpc_nft_storage
cd grpc_nft_storage
```

### **2. Install Dependencies**
Ensure you have Rust installed. If not, install it using:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Then install dependencies:
```sh
cargo build
```

### **3. Generate gRPC Code**
Ensure the `.proto` files are compiled into Rust code:
```sh
cargo clean
cargo build
```

## 🚀 Running the Server
Start the gRPC server with:
```sh
cargo run --bin server
```
By default, the server runs on `http://[::1]:50051`.

## 🛠️ API Endpoints
### **1. Store Metadata**
```proto
rpc StoreMetadata(MetadataRequest) returns (MetadataResponse);
```
Stores NFT metadata and returns the stored metadata.

### **2. Get Metadata**
```proto
rpc GetMetadata(TokenId) returns (MetadataResponse);
```
Retrieves metadata for a given token ID.

### **3. Update Metadata**
```proto
rpc UpdateMetadata(MetadataUpdateRequest) returns (MetadataResponse);
```
Updates existing metadata.

### **4. Delete Metadata**
```proto
rpc DeleteMetadata(TokenId) returns (Empty);
```
Deletes metadata for a given token ID.

### **5. List All Metadata**
```proto
rpc ListAllMetadata(Empty) returns (MetadataListResponse);
```
Returns all stored metadata.

## 📡 Running the Client
A sample client is available in the `client/` folder. Run it using:
```sh
cargo run --bin client
```

## 📜 License
MIT License

