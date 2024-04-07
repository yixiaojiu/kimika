#[derive(Default)]
pub struct RemoteService {}

// 尝试以下 thread_local
// https://doc.rust-lang.org/std/macro.thread_local.html
// https://doc.rust-lang.org/std/thread/struct.LocalKey.html

// #[tonic::async_trait]
// impl Remote for RemoteService {}
