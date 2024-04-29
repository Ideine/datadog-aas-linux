use std::env;
use std::process::Command;

fn main() {
    let dd_command: Vec<String> = env::args().collect();
    let mut command = Command::new(&dd_command[1]);

    // dogstatsd requires a start argument also including a declaration argument 
    // for config path -c <dogstatsd.yaml location>
    // trace-agent requires the args: run -c <datadog.yaml location>
    let dd_arg = command.get_program().to_str().unwrap();
    if dd_arg.ends_with("dogstatsd") || dd_arg.ends_with("trace-agent") {
        command.args(&dd_command[2..=4]);
    }

    spawn(command);
}

fn spawn(mut command: Command) {
    if let Ok(mut dd_process) = command.spawn() {
        let status = dd_process.wait().expect("dd_process wasn't running");
        println!("DataDog process {} has finished", status);
        if !status.success() {
            spawn(command);
        }
    } else {
        println!("Datadog process did not start successfully");
    }
}