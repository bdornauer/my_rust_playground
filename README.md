Good source: 
- https://blog.steelph0enix.dev/posts/llama-cpp-guide/#getting-a-model 

## Models 

- `.gguf` is a format used by `llama.cpp` to store model weights. It is a binary format that is optimized for fast loading and inference. 
- Selection of model is important for performance. 
  - Quantization (e.g. phi-4-q4.gguf --> 4 Bits with Integer) is a process of reducing the precision of the model weights to reduce the size of the model and speed up inference. Similiar for Floating Point precision (e.g. phi-4-fp16.gguf  --> 16 Bits with Floating Point).
  - `Instruct` are trained to for chat conversations others are trained for text generation.


## MyModels to test 
Useful was TheBlock: 
--- 
- Modells phi with 14 Million parameters from Microsoft: 
  - phi-4 with 4-bit quantization: https://huggingface.co/microsoft/phi-4-gguf/blob/main/phi-4-q4.gguf (9 GB)
  - phi-4 with 16-bit quantization: https://huggingface.co/microsoft/phi-4-gguf/blob/main/phi-4-fp16.gguf (30 GB) --> to much for M4, very slow with GPU

Providing prompts is done via ****: 
   - General Format: <|system|>Insert System Message<|end|><|user|>Insert User Message<|end|><|assistant|>
     - Systems: General information helping the LLM. 
     - User: The message from the user, e.g. "What is the capital of France?"
     - Assistant: The Format for the LLM to answer. 


--- 
- Mistral (with 7B and 14B parameters): 
  - mistral-7b-instruct-v0.2.Q5_K_M.gguf (large, recommended): https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF 
  - Mistral-Small-24B-Instruct-2501-Q4_K_M.gguf (larger, recommend for most use cases): https://huggingface.co/bartowski/Mistral-Small-24B-Instruct-2501-GGUF 

Providing prompts is done via:  
  - General Format: <s>[INST] Instruction [/INST] Model answer</s>[INST] Follow-up instruction [/INST]
  - `<s>` and `</s>` are special tokens for beginning of string (BOS) and end of string (EOS) while `[INST]` and `[/INST]` are regular strings.
  - Example: <s>[INST] What is your favorite condiment? [/INST] "Well, I'm quite partial to a good squeeze of fresh lemon juice. It adds just the right amount of zesty flavour to whatever I'm cooking up in the kitchen!"</s> [INST] The right amount of what? [/INST]


--- 
- Llama 3 (with 7B and 13B parameters):
  - Meta-Llama-3-8B-Instruct.Q5_K_M.gguf (large, recommend): https://huggingface.co/MaziyarPanahi/Meta-Llama-3-8B-Instruct-GGUF/blob/main/Meta-Llama-3-8B-Instruct.Q5_K_M.gguf
  - Meta-Llama-3-8B-Instruct.Q8 (large, recommend): https://huggingface.co/MaziyarPanahi/Meta-Llama-3-8B-Instruct-GGUF/blob/main/Meta-Llama-3-8B-Instruct.Q8_0.gguf 

Providing prompts is done via:
  - <|begin_of_text|>: Marks the beginning of the text.
  - <|eot_id|>: Marks the end of a message.
  <|start_header_id|>{role}<|end_header_id|>: Indicates the role of the message (system, user, or assistant).
  - <|end_of_text|>: Marks the end of the text.
  - {{ system_prompt }}:This is a placeholder for the system instruction. This is where you define the behavior of your chat assistant and/or the personality you would like your assistant to have. For example: "You are a helpful, respectful, and honest assistant. Always answer as helpfully as possible."
  - {{ user_message }}:This is a placeholder for the user’s input or question. When using the model, this would be replaced by the actual message or query from the user. For instance: “What’s the capital of France?”

Example: 

````
<|begin_of_text|>
<|start_header_id|>system<|end_header_id|>
You are a helpful assistant.
<|eot_id|>
<|start_header_id|>user<|end_header_id|>
What’s the capital of France?
<|eot_id|>
<|start_header_id|>assistant<|end_header_id|>
```