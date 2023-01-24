use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
struct Paragraph{
   paragraph:String,
}

#[derive(Serialize,Deserialize)]
struct Article{
    topic: String,
    author: String,
    paragraphs: Vec<Paragraph>

}

fn main() {

   let my_sample_data: Article = Article{
        
    topic : String::from("Howz is rust awesome"),
    author: String::from("Admin"),
    paragraphs: vec![
        Paragraph {
            paragraph: String::from("First Paragraph")
        },
        Paragraph {
            paragraph: String::from("Paragraph Body")
        },
        Paragraph {
            paragraph: String::from("Paragraph End")
        }
    ]
   };

   let json_data  = serde_json::to_string(&my_sample_data).unwrap();

    println!("JSON data is {}",json_data);
}
