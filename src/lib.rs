//! # LRAPIUT
//! This crate is a wrapper for the IUT La Rochelle's APIs.
//! <div class="warning">
//!     When using javascript bindings remember to replace the snake_case with camelCase.
//!     i.e. `get_credentials` becomes `getCredentials`.
//! </div>

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "notes")]
mod notes;

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
pub use crate::notes::NotesService;

#[macro_export]
/// Macro to expose an error to the javascript environment.
macro_rules! napi_error {
        ($emsg:tt) => {
           Err(napi::Error::new(napi::Status::GenericFailure, $emsg))
        };
}

/// Represents an IUT La Rochelle student.
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
#[doc(hidden)]
pub trait CoreFunctions {
    async fn get_cookies(&self) -> Result<()>;
    async fn try_get(&self, url: &str) -> Result<Value>;
}

#[napi]
impl LRUser {
    
    /// Creates a new LRUser.
    /// 
    /// # Arguments
    /// 
    /// * `username` - The student's username.
    /// * `password` - The student's password.
    ///
    /// # Examples
    /// ```
    /// use lrapiut::LRUser;
    ///
    /// let lrUser = LRUser::new("username".to_string(), "password".to_string());
/// ```
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
        #[cfg(feature = "notes")]
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

    /// Sets currents student's credentials.
    /// 
    /// # Arguments
    /// 
    /// * `username` - The new student's username.
    /// * `password` - The new student's password.
    ///
    /// # Examples
    /// ```
    /// use lrapiut::LRUser;
    ///
    /// let lrUser = LRUser::new("username".to_string(), "password".to_string());
    /// lrUser.set_credentials("new_username".to_string(), "new_password".to_string());
    /// ```
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

    
    /// Gets currents student's credentials.
    /// 
    /// <div class="warning">
    ///     This method is meant to be used in javascript only.
    /// </div>
    /// 
    /// # Examples
    /// ```
    /// use lrapiut::LRUser;
    ///
    /// let lrUser = LRUser::new("username".to_string(), "password".to_string());
    /// lrUser.get_credentials(/* `env` is injected by NAPI-RS */);
    /// ```
    /// 
    /// # Returns
    /// ```
    /// {"username": "username", "password": "password"}
    /// ```
    #[napi]
    pub fn get_credentials(&self, env: Env) -> Result<Object> {
        return Runtime::new()?.block_on(self._get_credentials(env));
    }
}

#[cfg_attr(feature = "notes", napi)]
#[cfg(feature = "notes")]
impl LRUser {
     /// Gets the notes service, it is used to access notes's endpoints.
    /// 
    /// <div class="warning">
    ///     This method does not need to be called with parenthesis in javascript as it is binded as a getter.
    /// </div>
    /// 
    /// # Examples
    /// ```
    /// use lrapiut::LRUser;
    ///
    /// let lrUser = LRUser::new("username".to_string(), "password".to_string());
    /// let notesService = lrUser.notes();
    /// ```
    #[napi(getter)]
    pub fn notes(&self) -> Arc<NotesService> {
        self._notes.clone()
    }
}