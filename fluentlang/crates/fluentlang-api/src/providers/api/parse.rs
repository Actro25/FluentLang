pub trait ApiParse {
    /// The function is supposed to return route to get content from a JSON file that was returned from server.
    fn json_content_route(&self) -> String;
}