use rand::Rng;
use std::io;
use std::process::Command;
use std::time::Duration;

// Function to generate a random math question and its correct answer based on the score
fn generate_question(score: u32) -> (String, f64) {
	let mut rng = rand::thread_rng();
	let max_value = 10 + score; // Increase max value for numbers based on score
	let operations = ["+", "-", "*", "/"]; // Array of possible operations
	let op = operations[rng.gen_range(0..operations.len())]; // Randomly select an operation
	let num1 = rng.gen_range(1..=max_value); // Generate first number
	let num2 = rng.gen_range(1..=max_value); // Generate second number

	// Generate the question and calculate the correct answer based on the operation
	let (question, answer) = match op {
		"/" => {
			// Ensure num1 is a multiple of num2 for division
			let num2 = rng.gen_range(1..=max_value);
			let num1 = num2 * rng.gen_range(1..=max_value);
			(format!("What is {} {} {}?", num1, op, num2), num1 as f64 / num2 as f64)
		},
		_ => (
			format!("What is {} {} {}?", num1, op, num2),
			match op {
				"+" => num1 as f64 + num2 as f64,
				"-" => num1 as f64 - num2 as f64,
				"*" => num1 as f64 * num2 as f64,
				_ => 0.0, // Default case, shouldn't happen
			}
		),
	};
	(question, answer)
}

// Function to clear the terminal screen based on the operating system
fn clear_screen() {
	#[cfg(target_os = "windows")]
	{
		// Clear screen for Windows
		let _ = Command::new("cmd")
			.args(&["/C", "cls"])
			.status();
	}

	#[cfg(not(target_os = "windows"))]
	{
		// Clear screen for non-Windows (Unix-based) systems
		let _ = Command::new("clear")
			.status();
	}
}

fn main() {
	let mut score = 0; // Initialize score

	loop {
		clear_screen(); // Clear screen at the start of each loop iteration
		let (question, correct_answer) = generate_question(score); // Generate a new question
		println!("{}", question); // Print the question to the user

		let mut user_input = String::new();
		io::stdin().read_line(&mut user_input).expect("Failed to read line"); // Read user input
		let user_input = user_input.trim().to_string();

		// Parse user input to a floating-point number
		let user_answer: f64 = match user_input.parse() {
			Ok(val) => val,
			Err(_) => {
				println!("Invalid input. Please enter a numeric value.");
				std::thread::sleep(Duration::from_secs(1)); // Wait before continuing
				continue;
			}
		};

		// Check if the user's answer is correct
		if (user_answer - correct_answer).abs() < 0.01 {
			score += 1; // Increment score for a correct answer
			println!("Correct! Your score is {}.", score);
		} else {
			println!("Incorrect. The correct answer was {:.2}.", correct_answer);
			println!("Game Over. Your final score is {}.", score);
			break; // End the game on incorrect answer
		}
		std::thread::sleep(Duration::from_secs(1)); // Wait before generating a new question
	}

	// Prompt the user to press Enter to exit
	println!("Press Enter to exit...");
	let mut pause_input = String::new();
	io::stdin().read_line(&mut pause_input).expect("Failed to read line");
}
