use std::process::ExitCode;

fn main() -> ExitCode {
    match crap4rust::run() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::from(2)
        }
    }
}
