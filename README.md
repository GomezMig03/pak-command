# Pak-command
Library to get os package manager or to know if system has specific commands, this was made just for fun and probably has no use in real production.

## Usage
### The ```package_manager``` function returns the system's package manager, if there is one.
#### Example
```
use pak_command::package_manager;
let pm: &str = package_manager();
println!("Your system's package manager is {}!", pm);
```
### The ```check_command``` function returns if the given command name exists in current system.
#### Example
```
use pak_command::check_command;
let first_command: &str = "cd";
let second_command: &str = "unexisting_command";
check_command(first_command); //true
check_command(second_command); //false
```