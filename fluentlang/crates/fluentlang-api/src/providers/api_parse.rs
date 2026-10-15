pub trait ApiParse {
    /// This function returns route to get content from a JSON file that was returned from server.
    fn json_content_route(&self) -> String;
}