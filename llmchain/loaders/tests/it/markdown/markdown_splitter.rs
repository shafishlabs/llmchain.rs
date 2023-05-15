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

use std::io::Write;

use anyhow::Result;
use goldenfile::Mint;
use llmchain_loaders::DocumentLoader;
use llmchain_loaders::DocumentSplitter;
use llmchain_loaders::MarkdownLoader;
use llmchain_loaders::MarkdownSplitter;
use opendal::services::Fs;
use opendal::BlockingOperator;
use opendal::Operator;

#[test]
fn test_markdown_splitter_default() -> Result<()> {
    // testdata dir.
    let curdir = std::env::current_dir()?.to_str().unwrap().to_string();
    let testdata_dir = format!("{}/tests/testdata", curdir);

    // Operator.
    let mut builder = Fs::default();
    builder.root("/");
    let op: BlockingOperator = Operator::new(builder)?.finish().blocking();

    // Load
    let markdown_loader = MarkdownLoader::create(op);
    let xx = format!("{}/markdown/copy.md", testdata_dir);
    let documents = markdown_loader.load(&xx)?;

    let markdown_splitter = MarkdownSplitter::create();
    let documents = markdown_splitter.split_documents(&documents)?;

    // Check.
    let mut mint = Mint::new(&testdata_dir);
    let golden_path = "markdown/copy_md_splitter_default.golden";
    let mut file = mint.new_goldenfile(golden_path)?;
    for (i, doc) in documents.iter().enumerate() {
        writeln!(
            file,
            "part={}, len={}, chunk_size={}, path={}",
            i,
            doc.content.len(),
            markdown_splitter.splitter_chunk_size,
            doc.meta.path
        )?;
        writeln!(
            file,
            "------------------------------------------------------------"
        )?;
        writeln!(file, "{}", doc.content)?;
    }

    Ok(())
}

#[test]
fn test_markdown_splitter_100() -> Result<()> {
    // testdata dir.
    let curdir = std::env::current_dir()?.to_str().unwrap().to_string();
    let testdata_dir = format!("{}/tests/testdata", curdir);

    // Operator.
    let mut builder = Fs::default();
    builder.root(&testdata_dir);
    let op: BlockingOperator = Operator::new(builder)?.finish().blocking();

    // Load
    let markdown_loader = MarkdownLoader::create(op);
    let documents = markdown_loader.load("markdown/copy.md")?;

    let markdown_splitter = MarkdownSplitter::create().with_chunk_size(100);
    let documents = markdown_splitter.split_documents(&documents)?;

    // Check.
    assert_eq!(documents.len(), 14);

    // Check.
    let mut mint = Mint::new(&testdata_dir);
    let golden_path = "markdown/copy_md_splitter_chunk_100.golden";
    let mut file = mint.new_goldenfile(golden_path)?;
    for (i, doc) in documents.iter().enumerate() {
        writeln!(
            file,
            "part={}, len={}, chunk_size={}, path={}",
            i,
            doc.content.len(),
            markdown_splitter.splitter_chunk_size,
            doc.meta.path
        )?;
        writeln!(
            file,
            "------------------------------------------------------------"
        )?;
        writeln!(file, "{}", doc.content)?;
    }

    Ok(())
}