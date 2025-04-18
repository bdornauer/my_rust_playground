//! Execut it with: cargo run -- <model_path> 
//! Model with Mistral: https://huggingface.co/TheBloke/MistralLite-7B-GGUF

use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::model::{AddBos, Special};
use llama_cpp_2::sampling::LlamaSampler;
use std::num::NonZeroU32;


#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
pub fn get_answer(prompt: String, model_path: String, documentInformation: Vec<String> ) -> String {

    let backend = LlamaBackend::init().unwrap();
    let params: LlamaModelParams = LlamaModelParams::default().with_n_gpu_layers(80); // -1 means all layers

    LlamaContextParams::default();

    let model =
        LlamaModel::load_from_file(&backend, model_path, &params).expect("unable to load model");

    let ctx_params = LlamaContextParams::default().with_n_ctx(Some(NonZeroU32::new(256).unwrap()));

    let mut ctx = model
        .new_context(&backend, ctx_params)
        .expect("unable to create the llama_context");

    // concatenate the prompt and the document information 
    // to create the final prompt

    let mut final_prompt = "QUESTION: ".to_string();
    final_prompt.push_str(&prompt);
    final_prompt.push_str("\n\n");

    final_prompt.push_str("CONTEXT: \n\n");
    for info in documentInformation {
        final_prompt.push_str(&info);
    }
    final_prompt.push_str("\n\n");

    println!("{}", final_prompt);

    let chunk_size = 1024; // Define chunk size
    let chunks: Vec<String> = final_prompt
        .as_bytes()
        .chunks(chunk_size)
        .map(|chunk| String::from_utf8_lossy(chunk).to_string())
                .collect();

    let mut output_total = String::new();

    for chunk in chunks {
        let tokens_list = model
            .str_to_token(&chunk, AddBos::Always)
            .unwrap_or_else(|_| panic!("failed to tokenize {chunk}"));

        let n_len = 64; // Adjusted length for each chunk

        // Create a llama_batch with size 1024
        let mut batch = LlamaBatch::new(64, 1);

        let last_index = tokens_list.len() as i32 - 1;

        for (i, token) in (0_i32..).zip(tokens_list.into_iter()) {
            let is_last = i == last_index;
            batch.add(token, i, &[0], is_last).unwrap();
        }

        ctx.decode(&mut batch).expect("llama_decode() failed");

        let mut n_cur = batch.n_tokens();

        // The `Decoder`
        let mut decoder = encoding_rs::UTF_8.new_decoder();
        let mut sampler = LlamaSampler::greedy();

        while n_cur <= n_len {
            let token = sampler.sample(&ctx, batch.n_tokens() - 1);
            sampler.accept(token);

            if token == model.token_eos() {
                eprintln!();
                break;
            }

            let output_bytes = model.token_to_bytes(token, Special::Tokenize).unwrap();
            let mut output_string = String::with_capacity(32);
            decoder.decode_to_string(&output_bytes, &mut output_string, false);

            output_total.push_str(&output_string);

            batch.clear();
            batch.add(token, n_cur, &[0], true).unwrap();

            n_cur += 1;
            ctx.decode(&mut batch).expect("failed to eval");
        }
    }

    output_total
}
