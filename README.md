# rust-grpc-web-react-template

This project is a minimal starter template to quickly get grpc-web working with a Rust backend and React frontend.
It takes care of proxying (Envoy not necessary), CORS, and generating types in TS and Rust.

If you prefer to use a framework other than React, it should be easy enough to change (just keep the scripts in `package.json`)

### Adding a new service

This project comes with an `EchoService` as a template. To create new services:
1. Add a new proto file in `grpc-backend/proto`
2. Add this file to `grpc-backend/build.rs`
3. Implement the service in `grpc-backend/src/services`
4. Add the service to the server in `grpc-backend/main.rs` 
5. Run `npm proto:generate` on the client
6. Call the service on the client! (see `client/src/App.tsx`)

