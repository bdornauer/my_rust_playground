use llama_cpp::{LlamaModel, LlamaParams, SessionParams};
use llama_cpp::standard_sampler::StandardSampler;
use std::io::{self, Write};

fn predict(){

    //let model_path = "./models/mistrallite_Q5_K_M.gguf";
    //let model_path = "./models/llama-2-7b.Q4_K_M.gguf";
    let model_path = "./models/DeepSeek-R1-Distill-Qwen-7B-Q8_0.gguf";

   let llama_params = LlamaParams {
        n_gpu_layers: 100,          // No GPU layers if you're using CPU (set to >0 if using GPU)
        main_gpu: 1,              // Main GPU index (set to 0 if you're not using a GPU)
       ..LlamaParams::default()  // Default values for other fields
    };


    let model = LlamaModel::load_from_file(model_path, llama_params)
        .expect("Failed to load model from file");

    // Session 
    let mut ctx = model.create_session(SessionParams::default())
        .expect("Failed to create session");

    let input_text = "<|prompter|>Who was the president from 1994 till 2025 the united states ?</s><|assistant|>";
    ctx.advance_context(input_text).unwrap();

    let max_tokens = 1024;
    let mut decoded_tokens = 0;
    
    let completions = ctx.start_completing_with(StandardSampler::default(), max_tokens)
        .expect("Failed to start completion")
        .into_strings();

    // Print each generated token
    for completion in completions {
        print!("{}", completion);
        io::stdout().flush().expect("Failed to flush stdout");

        decoded_tokens += 1;

        if decoded_tokens >= max_tokens {
            break;
        }
    }
}

fn main() {
    predict();
}
