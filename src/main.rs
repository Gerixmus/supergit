use inquire::MultiSelect;

fn main() {
    let options = vec![
        "Option 1",
        "Option 2",
        "Option 3"
    ];

    let selected_options = MultiSelect::new("Select your options", options).prompt();

    match selected_options {
        Ok(choices) => {
            if choices.is_empty() {
                println!("No options selected.");
            } else {
                println!("You selected:");
                for choice in choices {
                    println!("- {}", choice);
                }
            }
        }
        Err(err) => println!("An error occurred: {}", err),
    }   
}
