use crate::rest_methods::RESTMethods;

#[derive(Debug)]
pub struct OverpassAPI<U: reqwest::IntoUrl + Clone> {
    url: U,
    client: reqwest::blocking::Client,
}

impl<U: reqwest::IntoUrl + Clone> OverpassAPI<U> {
    pub fn new(url: U) -> Self {
        Self {
            url,
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl<U: reqwest::IntoUrl + Clone> RESTMethods for OverpassAPI<U> {
    type RequestBuilder = reqwest::blocking::RequestBuilder;

    fn get(&self) -> Self::RequestBuilder {
        self.client.get(self.url.clone())
    }

    fn post(&self) -> Self::RequestBuilder {
        self.client.post(self.url.clone())
    }

    fn put(&self) -> Self::RequestBuilder {
        self.client.put(self.url.clone())
    }

    fn patch(&self) -> Self::RequestBuilder {
        self.client.patch(self.url.clone())
    }

    fn delete(&self) -> Self::RequestBuilder {
        self.client.delete(self.url.clone())
    }

    fn head(&self) -> Self::RequestBuilder {
        self.client.head(self.url.clone())
    }
}
