pub trait RESTMethods {
    type RequestBuilder;

    fn get(&self) -> Self::RequestBuilder;

    fn post(&self) -> Self::RequestBuilder;

    fn put(&self) -> Self::RequestBuilder;

    fn patch(&self) -> Self::RequestBuilder;

    fn delete(&self) -> Self::RequestBuilder;

    fn head(&self) -> Self::RequestBuilder;
}
