type ExitCodeType = i32;

const SUCCESS: ExitCodeType = 0;
const WARNING: ExitCodeType = 0;
const ERROR: ExitCodeType = 1;

pub enum ExitCode {
    SUCCESS,
    WARNING,
    ERROR,
}

pub fn exit_command(exit_code: ExitCode) {
    match exit_code {
        ExitCode::SUCCESS => {
            std::process::exit(SUCCESS);
        }
        ExitCode::WARNING => {
            std::process::exit(WARNING);
        }
        ExitCode::ERROR => {
            std::process::exit(ERROR);
        }
    }
}
