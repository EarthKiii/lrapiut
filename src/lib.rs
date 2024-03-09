pub mod notes;
pub mod gpu;



use reqwest::{Client, ClientBuilder};
use napi::bindgen_prelude::*;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::runtime::Runtime;
use reqwest_cookie_store::CookieStoreMutex;
use napi_derive::napi;
use async_trait::async_trait;
#[cfg(feature = "notes")]
use crate::notes::notes::NotesService;

#[macro_export]
macro_rules! napi_error {
        ($emsg:tt) => {
           Err(napi::Error::new(napi::Status::GenericFailure, $emsg))
        };
}

#[napi]
pub struct LRUser {
    _client: Arc<Mutex<Client>>,
    _cookie_store: Arc<CookieStoreMutex>,
    username_notes: Arc<Mutex<String>>,
    password_notes: Arc<Mutex<String>>,
    #[cfg(feature = "notes")]
    _notes: Arc<NotesService>,
}

#[async_trait]
pub trait CoreFunctions {
    async fn get_cookies(&self) -> Result<()>;
    async fn try_get(&self, url: &str) -> Result<Value>;
}

#[napi]
impl LRUser {
    
    #[napi(constructor)]
    pub fn new(username: String, password: String) -> Self {
        let cookie_store = Arc::new(CookieStoreMutex::default());

        let client_ref = Arc::new(Mutex::new(ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .cookie_provider(cookie_store.clone())
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36")
        .build().unwrap()));

        let username_ref = Arc::new(Mutex::new(username));
        let password_ref = Arc::new(Mutex::new(password));
        let notes_service = Arc::new(NotesService::_new(client_ref.clone(), username_ref.clone(), password_ref.clone()));

        Self {
            _client: client_ref.clone(),
            _cookie_store: cookie_store,
            username_notes: username_ref.clone(),
            password_notes: password_ref.clone(),
            #[cfg(feature = "notes")]
            _notes: notes_service
        }
    }

    async fn _set_credentials(&self, username: String, password: String) -> Result<()> {
        *(self.username_notes.lock().await) = username;
        *(self.password_notes.lock().await) = password;
        self._cookie_store.lock().or(napi_error!("Unable to clear authentication cookies"))?.clear();
        Ok(())
    }

    #[napi]
    pub fn set_credentials(&self, username: String, password: String) -> Result<()> {
        let _ = Runtime::new()?.block_on(self._set_credentials(username, password));
        Ok(())
    }

    async fn _get_credentials(&self, env: Env) -> Result<Object> {
        let mut obj = env.create_object()?;
        obj.set("username", self.username_notes.lock().await.to_string())?;
        obj.set("password", self.password_notes.lock().await.to_string())?;
        Ok(obj)
    }

    #[napi]
    pub fn get_credentials(&self, env: Env) -> Result<Object> {
        return Runtime::new()?.block_on(self._get_credentials(env));
    }

    #[napi(getter)]
    #[cfg(feature = "notes")]
    pub fn notes(&self) -> Arc<NotesService> {
        self._notes.clone()
    }
}