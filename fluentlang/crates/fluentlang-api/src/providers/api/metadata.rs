use serde_json::Value;

pub const PROMPT: &str = "You are a professional language assistant for the language in the \
                        sentence below. You must analyze the grammar used in the sentence and point \
                        out any errors, if there are any. Respond concisely, in no more than 5 sentences, \
                        and only in the language of the sentence below.";

/// This trait have to be implemented for structs that want to use ApiBuildRequest trait.
/// The trait was created only for metadata to send request.
pub trait ApiMetaData {
    /// The function is supposed to return url for the api call.
    /// It shouldn't have parameters, because parameters will be entered when implementing ApiBuilderRequest
    fn url(&self) -> &str;

    /// The function is supposed build body.
    ///
    /// * `sentence` - a sentence that the user wants to know all about.
    fn body(&self, sentence: &str) -> Value;
}