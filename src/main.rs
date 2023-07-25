use reqwest::Client;
use select::document::Document;
use select::predicate::Name;
use serde_json::Value;

const LOGIN_URL: &str = "https://authentification.univ-lr.fr/cas/login?service=https://notes.iut-larochelle.fr/services/doAuth.php?href=https://notes.iut-larochelle.fr/";

pub struct LRUser {
    client: Client,
    username: String,
    password: String,
}

impl LRUser {
    
    pub fn new(username: String, password: String) -> Self {
        Self {
            client: Client::builder()
                .danger_accept_invalid_certs(true)
                .cookie_store(true)
                .build().unwrap(),
            username,
            password
        }
    }

    async fn get_cookies(&self) -> () {
        let pre_auth: String = self.client.get(LOGIN_URL)
            .send().await.unwrap()
            .text().await.unwrap();
    
        // Utiliser la bibliothèque select pour extraire la valeur d'exécution
        let document: Document = Document::from(pre_auth.as_str());
        let exec_value: &str = document
            .find(Name("input"))
            .filter(|n| n.attr("name").unwrap_or("") == "execution")
            .next()
            .and_then(|n: select::node::Node<'_>| n.attr("value")).unwrap();
    
        // Données du formulaire
        let form_data: [(&str, &str); 5] = [
            ("username", self.username.as_str()),
            ("password", self.password.as_str()),
            ("execution", exec_value),
            ("_eventId", "submit"),
            ("geolocation", ""),
        ];
    
        // Effectuer une requête POST pour se connecter
        self.client.post(LOGIN_URL)
            .form(&form_data)
            .send().await.unwrap();
    }

    async fn login(&self) -> () {
        let json_response: Value = self.client.get("https://notes.iut-larochelle.fr/services/data.php?q=donnéesAuthentification")
            .send().await.unwrap()
            .json().await.unwrap();

        if json_response.get("redirect").is_some()
        {
            self.get_cookies().await;
        }
    }

    pub async fn semestre_etudiant(&self) -> Value {
        
        self.login().await;

        let json_response: Value = self.client.get("https://notes.iut-larochelle.fr/services/data.php?q=semestresEtudiant")
            .send().await.unwrap()
            .json().await.unwrap();

        return json_response;
    }

    pub async fn data_premiere_connexion(&self) -> Value {
        
        self.login().await;

        let json_response: Value = self.client.get("https://notes.iut-larochelle.fr/services/data.php?q=dataPremièreConnexion")
            .send().await.unwrap()
            .json().await.unwrap();

        return json_response;
    }

    pub async fn releve_etudiant(&self, semestre: i64) -> Value {
        
        self.login().await;

        let semestres = self.semestre_etudiant();

        let mut formsemester_id: Option<String> = None;
        for semestre_json in semestres.await.as_array().unwrap() {
            if semestre_json.get("semestre_id").unwrap().as_i64().unwrap() == semestre {
                formsemester_id = Some(semestre_json.get("formsemestre_id").unwrap().as_i64().unwrap().to_string().to_owned());
            }
        }
        if formsemester_id.is_none() {
            panic!("Semestre introuvable");
        }

        let json_response: Value = self.client.get("https://notes.iut-larochelle.fr/services/data.php?q=relevéEtudiant&semestre=".to_owned()+&formsemester_id.unwrap())
            .send().await.unwrap()
            .json().await.unwrap();

        return json_response;
    }

    pub async fn delete_student_pic(&self) -> Value {
        
        self.login().await;

        let json_response: Value = self.client.get("https://notes.iutmulhouse.uha.fr/services/data.php?q=deleteStudentPic")
            .send().await.unwrap()
            .json().await.unwrap();

        return json_response;
    }

    pub async fn get_student_pic(&self) -> String {
        
        self.login().await;

        let json_response: String = self.client.get("https://notes.iutmulhouse.uha.fr/services/data.php?q=getStudentPic")
            .send().await.unwrap()
            .text().await.unwrap();

        return json_response;
    }

    pub async fn set_student_pic(&self) -> Value {

        return Value::String("Not imlemented yet".to_string());
    }

    pub async fn donnees_authentification(&self) -> Value {
        
        self.login().await;

        let json_response: Value = self.client.get("https://notes.iut-larochelle.fr/services/data.php?q=donnéesAuthentification")
            .send().await.unwrap()
            .json().await.unwrap();

        return json_response;
    }

}

fn main() {}
