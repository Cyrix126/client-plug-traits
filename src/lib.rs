#![allow(async_fn_in_trait)]
use cfg_if::cfg_if;
use url::Url;

cfg_if! {
if #[cfg(feature="product_api_client")] {
    pub trait Product {}

    pub trait ProductClient
    where
        Self: Sized,
    {
        fn new(uri: Url) -> Self;
        async fn get_product_from_id(&self, id: u32) -> Result<impl Product, impl std::error::Error>;
        async fn get_label_from_id(&self, id: u32) -> Result<String, impl std::error::Error>;
        async fn get_all_products(&self) -> Result<Vec<u32>, impl std::error::Error>;
    }

    }

}
cfg_if! {
if #[cfg(feature="tasks_tracker_api_client")] {
    // how to not duplicate the trait here but keep required trait on the where clause only for the feature bincode ?
cfg_if! {

if #[cfg(feature = "bincode")] {
use bincode::{Decode, Encode};
    pub trait NewTask where
    Self: Encode + Decode
    {
        fn duration(&self) -> u32;
        fn scope(&self) -> &str;
        fn name(&self) -> &str;
        fn description(&self) -> &str;
        fn push_address(&self) -> &[Url];
        fn payload(&self) -> &[u8];
    }

} else {
    pub trait NewTask where
    {
        fn duration(&self) -> u32;
        fn scope(&self) -> &str;
        fn name(&self) -> &str;
        fn description(&self) -> &str;
        fn push_address(&self) -> &[Url];
        fn payload(&self) -> &[u8];
    }

}
}
    pub trait ResponseNewTask  {
        fn location(&self) -> &Url;
        fn view_token(&self) -> &str;
        fn update_token(&self) -> &str;
        fn abort_token(&self) -> &str;
    }

    pub trait Task {}
    pub trait TaskTrackerClient
    where
        Self: Sized,
        {
            /// uri can contain a password, that will be put in the right header by different implementations.
            fn new(uri: Url) -> Self;
            async fn create_task(&self, new_task: impl NewTask, token: Option<&str>) -> Result<impl ResponseNewTask, impl std::error::Error>;
            async fn create_simple_task(&self, task_scope: String, task_name: String, token: Option<&str>) -> Result<impl ResponseNewTask, impl std::error::Error>;
            async fn update_task_progress(&self, task_location: &Url, new_progress: u8, token: Option<&str>) -> Result<(), impl std::error::Error>;
            async fn finish_task(&self, task_location: &Url, description_result: Option<&str>, payload_result: &[u8], token:Option<&str>) -> Result<(), impl std::error::Error>;
            async fn abort_task(&self, task_location: &Url, description_result: Option<&str>, payload_result: &[u8], token:Option<&str>) -> Result<(), impl std::error::Error>;
            async fn get_task(&self, task_location: &Url, token: Option<&str>) -> Result<impl Task, impl std::error::Error>;
        }
    }
}
cfg_if! {
if #[cfg(feature="cache_api_client")] {
    pub trait CacheClient
    where
        Self: Sized,
        {
            fn new(uri: Url, host: &str) -> Self;
            async fn delete_entry(&self, path: &str) -> Result<(), impl std::error::Error>;
        }
    }
}
