pub fn prompt_password(prompt: &str) -> Result<String, String> {
    eprint!("{}", prompt);
    rpassword::read_password().map_err(|e| e.to_string())
}
