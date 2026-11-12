use serde_json::Value;

pub const PROMPT: &str = "You are a professional language learning assistant. \
    Analyze EVERY word in the sentence provided below, in its original order. \
    For each word, provide the following information: \
    1. Word: The exact word from the sentence. \
    2. Part of speech: Identify its grammatical category (noun, verb, adjective, etc.). \
    3. Meaning: Explain what the word means in the context of the sentence. \
    4. Grammar: Briefly explain its grammatical form and function in the sentence, \
    including tense, number, case, or other relevant features when applicable. \
    Do not skip, combine, or repeat words, even if the same word appears multiple times. \
    After analyzing all words, briefly explain the overall meaning of the sentence \
    and point out any grammatical errors, if present. \
    Format your response using Markdown, with a separate numbered section for each word. \
    Keep explanations clear, concise, and suitable for language learners. \
    CRITICAL: Respond EXCLUSIVELY in the same language as the input sentence. \
    Do not use any other language.";

/// This trait have to be implemented for structs that want to use ApiBuildRequest trait.
/// The trait was created only for metadata to send request.
pub trait ApiMetaData {
    /// The function is supposed to return url for the api call.
    /// It shouldn't have parameters, because parameters will be entered when implementing ApiBuilderRequest
    fn url(&self) -> &str;

    /// The function is supposed build body.
    ///
    /// # Arguments
    /// 
    /// * `sentence` - a sentence that the user wants to know all about.
    fn body(&self, sentence: &str) -> Value;
}