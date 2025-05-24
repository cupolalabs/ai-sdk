use ai_providers::{
    openai::constants::OpenAIModelId, openai::request::input::Input, OpenAIProvider, OpenAIRequest,
    ProviderStrategy,
};

#[tokio::test]
async fn test_it_works() {
    let api_key =
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");
    let provider = OpenAIProvider::new(api_key);

    let request = OpenAIRequest::new(
        OpenAIModelId::Gpt3_5Turbo,
        Input::Message("Who's Fred again..".into()),
    );

    let result = provider.generate(&request).await.unwrap();

    println!("{:?}", result);

    assert_eq!("test", "test");
}
