use tonic::{Response, Status};

pub type TonicRes<T> = Result<Response<T>, Status>;
