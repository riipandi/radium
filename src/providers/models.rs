/// ModelProvider represents the different AI model providers supported by Radium.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelProvider {
    OpenAI,
    Anthropic,
    AzureOpenAI,
    Bedrock,
    Cohere,
    Vertex,
}
