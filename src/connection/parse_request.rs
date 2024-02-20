use std::collections::HashSet;

pub fn parse_request(request: &str) -> String {
    // TODO: Modify the parse_request function to parse the request taking into account the Redis protocol
    // https://redis.io/docs/reference/protocol-spec/
    const CONTROL_CHARACTERS: [char; 5] = ['+', '-', ':', '$', '*'];
    let mut parsed_request = String::new();
    let mut command_found = false;
    let commands: HashSet<&str> = HashSet::from(["PING", "ECHO", "SET", "GET"]);
    request.split("\r\n").for_each(|line| {
        let mut pass = false;
        for c in CONTROL_CHARACTERS.iter() {
            if line.starts_with(*c) {
                pass = true;
                break;
            }
        }
        if !pass {
            if !command_found && commands.contains(line.to_uppercase().as_str()) {
                command_found = true;
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

        let request = "$3\r\nset\r\n$10\r\nwatermelon\r\n$5\r\nvalue\r\n";
        assert_eq!(parse_request(request), "SET watermelon value");
    }
}