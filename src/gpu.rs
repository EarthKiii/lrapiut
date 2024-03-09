#[cfg(feature = "gpu")]
pub mod gpu {

    use crate::LRUser;
    use crate::Document;
    use crate::Name;
    use crate::Value;
    use crate::CoreFunctions;
    use crate::async_trait;

    #[async_trait(?Send)]
    impl CoreFunctions for LRUser {
        async fn get_cookies(&self) {
            let form_data: [(&str, &str); 3] = [
                ("util", self.username.as_str()),
                ("acct_pass", self.password.as_str()),
                ("modeconnect", "connect")
            ];
        
            self.client.post("https://www.gpu-lr.fr/sat/index.php?page_param=accueilsatellys.php")
                .form(&form_data)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .send().await.unwrap();

            self.client.get("https://www.gpu-lr.fr/gpu/index.php")
                .send().await.unwrap();
        }
    
        async fn login(&self) {
            let body: String = self.client.get("https://www.gpu-lr.fr/gpu/index.php?page_param=accueil.php")
                .send().await.unwrap()
                .text().await.unwrap();

            let document = Document::from(body.as_str());

            // Find the title element using the `select` crate
            let title = document
                .find(Name("title"))
                .next()
                .map(|title_element| title_element.text())
                .unwrap_or_default();
    
            if title.contains("404") {
                self.get_cookies().await;
            }
        }
    }

    impl LRUser {
    
    
        pub async fn vcs(&self, week: i8) -> String {
            self.login().await;
    
            let vcs_response: String = self.client.get("https://www.gpu-lr.fr/gpu/gpu2vcs.php?semaine=".to_owned()+&week.to_string()+"&prof_etu=ETU&etudiant="+&self.username.to_string()+"&enseignantedt=")
                .send().await.unwrap()
                .text().await.unwrap();
    
            return vcs_response;
        }
    
    }
    
}


// ========================
// Tests
// ========================
#[cfg(all(test, feature = "gpu"))]
mod tests {
    use crate::LRUser;
    use tokio;

    #[tokio::test] 
    async fn gpuclient() {
        let res = LRUser::new("220166".to_string(), "123".to_string());
        let b= res.vcs(39).await;
        println!("Res of test: {}", b);
    }
}