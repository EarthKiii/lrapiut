#[cfg(feature = "notes")]
pub mod notes {

    use napi::bindgen_prelude::*;
    use napi_derive::napi;
    use std::{sync::mpsc};
    use std::thread;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use reqwest::{header::{self, HeaderValue}, Client, Response};
    use serde_json::{from_str, Value, from_slice};
    use crate::CoreFunctions;
    use async_trait::async_trait;
    use crate::napi_error;

    const LOGIN_URL: &str = "https://authentification.univ-lr.fr/cas/login?service=https://notes.iut-larochelle.fr/services/doAuth.php?href=https://notes.iut-larochelle.fr/";

    #[napi]
    #[derive(Clone)]
    pub struct NotesService {
        client: Arc<Mutex<Client>>,
        username: Arc<Mutex<String>>,
        password: Arc<Mutex<String>>
    }

    #[napi]
    #[async_trait]
    impl CoreFunctions for NotesService {

        async fn get_cookies(&self) -> Result<()> {
            let pre_auth: String = self.client.lock().await.get(LOGIN_URL)
                .send().await.unwrap()
                .text().await.unwrap();
    
            let (tx, rx) = mpsc::channel::<String>();

            let handle = thread::spawn( move || -> Result<()> {
                use tl::{parse, ParserOptions};
                const EMSG: &str = "Unable to extract execution code for Authentification";

                let css_selector = "input[name=\"execution\"]";
                let dom = parse(&pre_auth, ParserOptions::default()).or(napi_error!(EMSG))?;
                let parser = dom.parser();
                let mut element = match dom.query_selector(css_selector) {
                    Some(element) => element,
                    None => return napi_error!(EMSG),
                };

                let node = match element.next() {
                    Some(node) => node,
                    None => return napi_error!(EMSG),
                };

                let tag = match node.get(parser) {
                    Some(tag) => tag,
                    None => return napi_error!(EMSG),
                };

                let attributes = match tag.as_tag() {
                    Some(tag) => tag.attributes(),
                    None => return napi_error!(EMSG),
                };

                let value = match attributes.get("value") {
                    Some(value) => value,
                    None => return napi_error!(EMSG),
                };

                let value_str = match value {
                    Some(value_str) => value_str.as_utf8_str().into_owned(),
                    None => return napi_error!(EMSG),
                };
                let _ = tx.send(value_str);
                Ok(())
            });

            let _ = handle.join().or(napi_error!("Unable to extract execution code for Authentification"))?;
            let res = rx.recv().or(napi_error!("Unable to extract execution code for Authentification"))?;

            let username = self.username.lock().await.to_owned();
            let password = self.password.lock().await.to_owned();

            let form_data: [(&str, &str); 5] = [
                ("username", username.as_str()),
                ("password", password.as_str()),
                ("execution", res.as_str()),
                ("_eventId", "submit"),
                ("geolocation", ""),
            ];

            self.client.lock().await.post(LOGIN_URL)
                .form(&form_data)
                .send().await.or(napi_error!("Authentification failed"))?;

            Ok(())
        }

        async fn try_get(&self, url: &str) -> Result<Value> {
            let response: Response = match self.client.lock().await.get(url)
            .send().await {
                Ok(res) => res,
                Err(e) => {let error_string = e.to_string(); return napi_error!(error_string)}
            };

            let text_plain = HeaderValue::from_static("text/plain");
            let content_type = response.headers().get(header::CONTENT_TYPE).unwrap_or(&text_plain);

            let mut json_response = if content_type == HeaderValue::from_static("application/json") {
                    match response.json().await {
                        Ok(json) => json,
                        Err(e) => {let error_string = e.to_string(); return napi_error!(error_string)}
                    }
                }
                else {
                    match response.bytes().await {
                        Ok(bytes) => from_slice(&bytes).unwrap_or(Value::default()),
                        Err(e) => {let error_string = e.to_string(); return napi_error!(error_string)}
                    }
            };


            if json_response.get("redirect").is_some()
            {
                let _ = self.get_cookies().await;
                let response: Response = match self.client.lock().await.get(url)
                .send().await {
                    Ok(res) => res,
                    Err(e) => {let error_string = e.to_string(); return napi_error!(error_string)}
                };

                let text_plain = HeaderValue::from_static("text/plain");
                let content_type = response.headers().get(header::CONTENT_TYPE).unwrap_or(&text_plain);

                json_response = if content_type == HeaderValue::from_static("application/json") {
                        match response.json().await {
                            Ok(json) => json,
                            Err(e) => {let error_string = e.to_string(); return napi_error!(error_string)}
                        }
                    }
                    else {
                        match response.bytes().await {
                            Ok(bytes) => from_slice(&bytes).unwrap_or(Value::default()),
                            Err(e) => {let error_string = e.to_string(); return napi_error!(error_string)}
                        }
                };
            }

            if json_response.get("redirect").is_some()
            {
                return napi_error!("Authentification failed");
            }

            Ok(json_response)
        }
    }

    #[napi]
    impl NotesService {

        pub fn _new(client: Arc<Mutex<Client>>, username: Arc<Mutex<String>>, password: Arc<Mutex<String>>) -> Self {
            Self {
                client,
                username,
                password
            }
        }

        #[napi(constructor)]
        pub fn new(username: String, password: String) -> Self {
            Self {
                client: Arc::new(Mutex::new(Client::new())),
                username: Arc::new(Mutex::new(username)),
                password: Arc::new(Mutex::new(password))
            }
        }

 
        #[napi]
        pub async fn semestre_etudiant(&self) -> Result<Value> {
            let json_response: Value = self.try_get("https://notes.iut-larochelle.fr/services/data.php?q=semestresEtudiant").await?;
            Ok(json_response)
        }

        #[napi]
        pub async fn data_premiere_connexion(&self) -> Result<Value> {
            let json_response: Value = self.try_get("https://notes.iut-larochelle.fr/services/data.php?q=dataPremièreConnexion").await?;
            Ok(json_response)
        }
    
        #[napi]
        pub async fn releve_etudiant(&self, semestre: i64) -> Result<Value> {
            let mut formsemester_id: Option<String> = None;

            if semestre <= 6 {
                let semestres = self.semestre_etudiant();
    
                for semestre_json in semestres.await?.as_array().unwrap() {
                    if semestre_json.get("semestre_id").unwrap().as_i64().unwrap() == semestre {
                        formsemester_id = Some(semestre_json.get("formsemestre_id").unwrap().as_i64().unwrap().to_string().to_owned());
                    }
                }
            } else {
                formsemester_id = Some(semestre.to_string());
            }

            if formsemester_id.is_none() {
                panic!("Semestre introuvable");
            }

            let json_response: Value = self.try_get(format!("{}{}","https://notes.iut-larochelle.fr/services/data.php?q=relevéEtudiant&semestre=",&formsemester_id.unwrap()).as_str()).await?;
            Ok(json_response)
        }

        #[napi]
        pub async fn delete_student_pic(&self) -> Result<Value> {
            let json_response: Value = self.try_get("https://notes.iut-larochelle.fr/services/data.php?q=deleteStudentPic").await?;
            Ok(json_response)
        }

        #[napi]
        pub async fn get_student_pic(&self) -> Result<Value> {
            let json_response: Value = self.try_get("https://notes.iut-larochelle.fr/services/data.php?q=getStudentPic").await?;
            Ok(json_response)
        }

        #[napi]
        pub async fn set_student_pic(&self) -> Result<Value> {
            let json_response: Value = from_str("Not imlemented yet").unwrap();
            Ok(json_response)
        }

        #[napi]
        pub async fn donnees_authentification(&self) -> Result<Value> {
            let json_response: Value = self.try_get("https://notes.iut-larochelle.fr/services/data.php?q=donnéesAuthentification").await?;
            Ok(json_response)
        }
    }
}

// ========================
// Tests
// ========================
#[cfg(all(test, feature = "notes"))]
mod tests {
    use crate::LRUser;
    use serde_json::from_str;
    use tokio;

    #[tokio::test] 
    async fn test_semestre() {
        let res = LRUser::new("username".to_string(), "password".to_string());

        let b = res.notes().semestre_etudiant().await;
        println!("Res of test: {}", b.unwrap_or(from_str("error").unwrap()));
    }

}