mod execute_model;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
        run_test();
    }

fn run_test() {
    let mut file = File::open("./src/example.json").expect("Unable to open file");
    
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Unable to read file");

    let unwrapped_data: Value = serde_json::from_str(&data).unwrap();
    let models = &unwrapped_data["test_set"];

    for model in models.as_array().unwrap() {
        let model_name = model["model_name"].as_str().unwrap();
        let model_path = model["model_path"].as_str().unwrap();
        
        delete_file_with_name(format!("./results/{}.md", model_name).as_str());

        for prompt in model["prompts"].as_array().unwrap() {
            let prompt_text = prompt.as_str().unwrap();

            save_string_to_file(format!("./results/{}.md", model_name).as_str(),format!("- **Prompt:** {}\n", prompt_text).as_str());

            let answer = execute_model::get_answer(prompt_text.to_string(), model_path.to_string());
            save_string_to_file(format!("./results/{}.md", model_name).as_str(),format!("- **Answer:** {}\n", answer).as_str());
            save_string_to_file(format!("./results/{}.md", model_name).as_str(),"---");
        }
    }
}


fn delete_file_with_name(filename: &str) -> std::io::Result<()> {
    std::fs::remove_file(filename)?;
    Ok(())
}

fn save_string_to_file(filename: &str, content: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;

    writeln!(file, "{}", content)?; 
    Ok(())
}
