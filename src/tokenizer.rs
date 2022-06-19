const SINGLETS: &str = ":=;+-*^/%(){}[]&.,|<>!";

pub enum Token{
    Symbol,
    Number,
    String,
    Blank,
    Singlet(char),
    Error(String)
}

pub enum SingletClass{
    Operator,
    Bracket,
    TypeSpecifier,
    Misc,
}

fn tokenize(input: &str) -> Vec<Token>{
    for c in input.chars(){
        
    }
}

#[cfg(test)]
mod tests{
    #[test]
    fn bruh(){

    }
}
