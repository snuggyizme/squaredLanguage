use crate::lexer::Token;

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

    fn parseProgram(&mut self) {
        let statements: Vec<Statement> = Vec::new();

        while ! self.isAtEnd() {
            
        }
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