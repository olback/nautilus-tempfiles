use tokio::runtime;

pub struct TokioRuntimeBuilder(runtime::Builder);

impl TokioRuntimeBuilder {

    pub fn new() -> Self {

        let mut inner = runtime::Builder::new();
        inner.thread_name("tempfiles-upload-thread-pool");
        inner.threaded_scheduler();
        inner.core_threads(2);
        inner.max_threads(4);
        inner.enable_all();

        Self(inner)

    }

    pub fn build(mut self) -> std::io::Result<runtime::Runtime> {
        self.0.build()
    }

}
