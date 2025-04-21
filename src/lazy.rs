macro_rules! lazy_async {
    ($async_expr:expr) => {
        ::once_cell::sync::Lazy::new(|| {
            let result = std::thread::spawn(|| {
                let rt = ::tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async { $async_expr })
            }).join().unwrap();
            result
        })
    };
}

macro_rules! statement {
    ($($stmt:expr), *) => {
        lazy_async! {
            crate::DB::prepare($($stmt),*)
            .await
            .expect("Couldn't prepare statement")
        }
    };
}

pub(super) use lazy_async;
pub(super) use statement;
