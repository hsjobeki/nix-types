#[macro_export]
#[rustfmt::skip]
macro_rules! T {
    ("/*")      => ($crate::SyntaxKind::TOKEN_MULTILINE_COMMENT_START);
    ("*/")      => ($crate::SyntaxKind::TOKEN_MULTILINE_COMMENT_END);
    (::)       => ($crate::SyntaxKind::TOKEN_DOUBLE_COLON);
    (|)       => ($crate::SyntaxKind::TOKEN_PIPE);

    
    (let)     => ($crate::SyntaxKind::TOKEN_LET);
    (in)      => ($crate::SyntaxKind::TOKEN_IN);
    
    ('{')     => ($crate::SyntaxKind::TOKEN_L_BRACE);
    ('}')     => ($crate::SyntaxKind::TOKEN_R_BRACE);
    ('[')     => ($crate::SyntaxKind::TOKEN_L_BRACK);
    (']')     => ($crate::SyntaxKind::TOKEN_R_BRACK);
    ('(')     => ($crate::SyntaxKind::TOKEN_L_PAREN);
    (')')     => ($crate::SyntaxKind::TOKEN_R_PAREN);

    (=)       => ($crate::SyntaxKind::TOKEN_ASSIGN);
    (@)       => ($crate::SyntaxKind::TOKEN_AT);
    (:)       => ($crate::SyntaxKind::TOKEN_COLON);
    (,)       => ($crate::SyntaxKind::TOKEN_COMMA);
    (.)       => ($crate::SyntaxKind::TOKEN_DOT);
    (...)     => ($crate::SyntaxKind::TOKEN_ELLIPSIS);
    (?)       => ($crate::SyntaxKind::TOKEN_QUESTION);
    (;)       => ($crate::SyntaxKind::TOKEN_SEMICOLON);
    (++)      => ($crate::SyntaxKind::TOKEN_CONCAT);
    (!)       => ($crate::SyntaxKind::TOKEN_INVERT);
    ("//")    => ($crate::SyntaxKind::TOKEN_UPDATE);

    (+)       => ($crate::SyntaxKind::TOKEN_ADD);
    (-)       => ($crate::SyntaxKind::TOKEN_SUB);
    (*)       => ($crate::SyntaxKind::TOKEN_MUL);
    (/)       => ($crate::SyntaxKind::TOKEN_DIV);

    (&&)      => ($crate::SyntaxKind::TOKEN_AND_AND);
    (==)      => ($crate::SyntaxKind::TOKEN_EQUAL);
    (->)      => ($crate::SyntaxKind::TOKEN_IMPLICATION);
    (<)       => ($crate::SyntaxKind::TOKEN_LESS);
    (<=)      => ($crate::SyntaxKind::TOKEN_LESS_OR_EQ);
    (>)       => ($crate::SyntaxKind::TOKEN_MORE);
    (>=)      => ($crate::SyntaxKind::TOKEN_MORE_OR_EQ);
    (!=)      => ($crate::SyntaxKind::TOKEN_NOT_EQUAL);
    (||)      => ($crate::SyntaxKind::TOKEN_OR_OR);
    ($kind:ident) => ($crate::SyntaxKind::$kind);
    
}
