// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_CLIENT_SEND_TRANSACTION: ::grpcio::Method<super::client::Transaction, super::client::TransactionReceipt> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Client/SendTransaction",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ClientClient {
    client: ::grpcio::Client,
}

impl ClientClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ClientClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn send_transaction_opt(&self, req: &super::client::Transaction, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::client::TransactionReceipt> {
        self.client.unary_call(&METHOD_CLIENT_SEND_TRANSACTION, req, opt)
    }

    pub fn send_transaction(&self, req: &super::client::Transaction) -> ::grpcio::Result<super::client::TransactionReceipt> {
        self.send_transaction_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_transaction_async_opt(&self, req: &super::client::Transaction, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::TransactionReceipt>> {
        self.client.unary_call_async(&METHOD_CLIENT_SEND_TRANSACTION, req, opt)
    }

    pub fn send_transaction_async(&self, req: &super::client::Transaction) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::TransactionReceipt>> {
        self.send_transaction_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Client {
    fn send_transaction(&mut self, ctx: ::grpcio::RpcContext, req: super::client::Transaction, sink: ::grpcio::UnarySink<super::client::TransactionReceipt>);
}

pub fn create_client<S: Client + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLIENT_SEND_TRANSACTION, move |ctx, req, resp| {
        instance.send_transaction(ctx, req, resp)
    });
    builder.build()
}
