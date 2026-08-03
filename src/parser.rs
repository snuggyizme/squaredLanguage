use crate::lexer::Token;
use crate::lexer::TokenType;

pub struct Statement {

}

pub struct Program {
    statements: Vec<Statement>,
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

#[allow(non_snake_case)]
impl Parser {
    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.index + offset)
    }
    fn advance(&mut self) {
        self.index += 1;
    }
    fn isAtEnd(&self) -> bool {
        self.index >= self.tokens.len()
    }

    pub fn parseProgram(&mut self) -> Program {
        let mut statements: Vec<Statement> = Vec::new();

        while ! self.isAtEnd() {
            statements.push(self.parseStatement());
        }

        Program {
            statements
        }
    }

    fn parseStatement(&mut self) -> Statement {
        let current: &Token = self.peek(0).unwrap();

        match current.tokenType {
            // Keywords
            TokenType::KeywordIf
            | TokenType::KeywordElse
            | TokenType::KeywordFor => {
                self.parseKeyword()
            }

            // Variable declaration
            TokenType::Identifier(name) => {
                let types: [String; 4] = [
                    String::from("int"),
                    String::from("str"),
                    String::from("vec2"),
                    String::from("range"),
                ];

                if types.contains(&name) {
                    self.parseVariableDeclaration()
                }
                
            } // This is to test my pushing ability // Dw we all know you're a great pusher :3

        }

        Statement {}
    }

    fn parseKeyword(&self) {
        todo!()
    }

    fn parseVariableDeclaration(&self) {
        todo!()
    }
}

#[allow(non_snake_case)]
pub fn parse(tokens: Vec<Token>) {
    let mut parser = Parser {
        tokens: tokens,
        index: 0,
    };
    parser.parseProgram();
    
}