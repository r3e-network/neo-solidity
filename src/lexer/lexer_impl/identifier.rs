impl Lexer {
    fn read_identifier(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut value = String::new();

        while self.position < self.input.len() {
            match self.current_char() {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    value.push(self.current_char());
                    self.advance();
                }
                _ => break,
            }
        }

        let token_type = match value.as_str() {
            "let" => TokenType::Let,
            "if" => TokenType::If,
            "for" => TokenType::For,
            "else" => TokenType::Else,
            "switch" => TokenType::Switch,
            "case" => TokenType::Case,
            "default" => TokenType::Default,
            "leave" => TokenType::Leave,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "function" => TokenType::Function,
            // Built-in functions
            "add" | "sub" | "mul" | "div" | "mod" | "eq" | "lt" | "gt" | "iszero" | "and"
            | "or" | "xor" | "not" | "byte" | "shl" | "shr" | "sar" | "keccak256" | "sha256"
            | "ripemd160" | "ecrecover" | "mload" | "mstore" | "sload" | "sstore" | "caller"
            | "callvalue" | "calldataload" | "calldatasize" | "calldatacopy" | "gas"
            | "gasprice" | "gaslimit" | "origin" | "address" | "balance" | "selfbalance"
            | "basefee" | "chainid" | "timestamp" | "number" | "difficulty" | "blockhash"
            | "coinbase" | "log0" | "log1" | "log2" | "log3" | "log4" | "create" | "create2"
            | "call" | "callcode" | "delegatecall" | "staticcall" | "return" | "revert"
            | "selfdestruct" => TokenType::BuiltinFunction,
            _ => TokenType::Identifier,
        };

        Token {
            token_type,
            value,
            line: start_line,
            column: start_column,
        }
    }
}

