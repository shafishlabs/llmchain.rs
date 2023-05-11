// Copyright 2023 Shafish Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::Result;
use llmchain_llms::llm::LLM;
use llmchain_llms::openai::OpenAI;
use llmchain_llms::openai::OpenAIConfig;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_llm_openai_generate() -> Result<()> {
    let key = std::env::var("OPENAI_API_KEY").unwrap_or("".to_string());
    let openai_conf = OpenAIConfig {
        openai_api_key: key,
        ..Default::default()
    };

    let openai_llm = OpenAI::create(openai_conf);
    let result = openai_llm.generate("say Hello").await?;
    let generation = result.generation;
    assert!(generation.contains("Hello"));
    assert_eq!(result.prompt_tokens, 10);
    assert_eq!(result.total_tokens, 19);
    assert_eq!(result.completion_tokens, 9);

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_llm_openai_embedding() -> Result<()> {
    let key = std::env::var("OPENAI_API_KEY").unwrap_or("".to_string());
    let openai_conf = OpenAIConfig {
        openai_api_key: key,
        ..Default::default()
    };

    let openai_llm = OpenAI::create(openai_conf);
    let inputs = vec!["llmchain".to_string(), "rs".to_string()];
    let result = openai_llm.embedding(inputs).await?;
    let embeddings = result.embeddings;
    assert_eq!(embeddings.len(), 2);

    assert_eq!(embeddings[0].len(), 1536);
    assert_eq!(embeddings[1].len(), 1536);
    assert_eq!(result.prompt_tokens, 4);
    assert_eq!(result.total_tokens, 4);

    Ok(())
}
