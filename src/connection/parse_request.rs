use std::collections::HashSet;

const CONTROL_CHARACTERS: [char; 5] = ['+', '-', ':', '$', '*'];
const CONTROL_COMMANDS: [&str; 5] = ["PING", "ECHO", "SET", "GET", "PX"];
const CRLF: &str = "\r\n";

pub fn parse_request(request: &str) -> String {
    // TODO: Modify the parse_request function to parse the request taking into account the Redis protocol
    // https://redis.io/docs/reference/protocol-spec/
    let mut parsed_request = String::new();
    let mut command_found = false;
    let commands: HashSet<&str> = HashSet::from(CONTROL_COMMANDS);
    request.split(CRLF).for_each(|line| {
        let mut pass = false;
        for c in CONTROL_CHARACTERS.iter() {
            if line.starts_with(*c) {
                pass = true;
                break;
            }
        }
        if !pass {
            if !command_found && commands.contains(line.to_uppercase().as_str()) {
                if line.to_uppercase() != "SET"{
                    command_found = true;
                }
                parsed_request = format!("{} {}", parsed_request, line.to_uppercase());
            } else {
                parsed_request = format!("{} {}", parsed_request, line);
            }
        }
    });
    parsed_request.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_request() {
        let request = "PING\r\n";
        assert_eq!(parse_request(request), "PING");

        let request = "ECHO\r\nHello\r\nWorld\r\n";
        assert_eq!(parse_request(request), "ECHO Hello World");

        let request = "SET\r\nkey\r\nvalue\r\n";
        assert_eq!(parse_request(request), "SET key value");

        let request = "GET\r\nkey\r\n";
        assert_eq!(parse_request(request), "GET key");

        let request = "PX\r\n1000\r\n";
        assert_eq!(parse_request(request), "PX 1000");

        let request: &str = "set\r\nwatermelon\r\nvalue\r\npx\r\n100\r\n";
        assert_eq!(parse_request(request), "SET watermelon value PX 100");

        let request = "$3\r\nset\r\n$10\r\nwatermelon\r\n$5\r\nvalue\r\n";
        assert_eq!(parse_request(request), "SET watermelon value");

        let request = "$3\r\nget\r\n$10\r\nwatermelon\r\n";
        assert_eq!(parse_request(request), "GET watermelon");
    }
}