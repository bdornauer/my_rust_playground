mod execute_model;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut document_information: Vec<String> = Vec::new();


    let path = "./europa-fakten-fischerei.pdf"; // Replace with your PDF file path
    let extracted_text = extract_pdf_information_enrich_llm(path);
    /*
    document_information.push("Fischen in Tirol ist ein beliebtes Hobby und eine wichtige Freizeitbeschäftigung. \n".to_string());
    document_information.push("Die Tiroler Gewässer sind reich an Fischarten und bieten zahlreiche Möglichkeiten zum Angeln. \n".to_string());
    document_information.push("Die Fischerei in Tirol unterliegt strengen Regelungen, um die Bestände zu schützen und die Umwelt zu erhalten. \n".to_string());
    document_information.push("Die Tiroler Fischereiverordnung regelt die Fischerei in den Gewässern des Landes. \n".to_string());
    document_information.push("Die Verordnung legt fest, welche Fischarten gefangen werden dürfen, welche Fangmethoden erlaubt sind und welche Schonzeiten gelten. \n".to_string());
    */

    document_information.push(extracted_text);
    
    let question = "Fasse alle Informationen neu zusammen in einem Satz auf Deutsch mit in 10 Worten. \n\n";

    let _answer = execute_model::get_answer(question.to_string(), "./models/llama/Llama-3-8B-Q5.gguf".to_string(), document_information);

    println!("Answer: {}", _answer);
    
}

fn extract_pdf_information_enrich_llm(path: &str) -> String {
    let bytes = std::fs::read(path).unwrap();
    let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();
    println!("Extracted text: {}", out);
    out
}
