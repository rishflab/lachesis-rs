//use fal::client::Client;
//use fal::transaction::TransactionStatus;

//struct LachesisClient<
//    T: Transaction,
//    U: Transport<W, X>,
//    W: Message,
//    X: TransportError> {
//
//}

struct LachesisClient {

}

struct HTTPTransport {

}
struct Message;

struct TransportError;

struct Transaction;

//impl Client for LachesisClient {

impl LachesisClient {

    fn submit_transaction(tx_hash: [u8; 32], tx: Transaction) -> TransactionStatus {
        unimplemented!()
    }

    fn check_transaction_status(tx_hash: [u8; 32]) -> TransactionStatus {
        unimplemented!()
    }

}

impl LachesisClient {



}
