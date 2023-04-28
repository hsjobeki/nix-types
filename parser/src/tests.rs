extern crate rnix;
use rnix::{
    ast::{self, AstToken},
    match_ast,
    // match_ast,
    SyntaxKind::{self, TOKEN_WHITESPACE},
    SyntaxToken,
};
// use crate::match_ast;

use expect_test::expect_file;
use std::{ffi::OsStr, fmt::Write, fs, path::PathBuf};

// use expect_test::expect_file;
// use rowan::ast::AstNode;

// use crate::ast::{self, AstToken, HasEntry};
use crate::{tokenize, Root};

// macro_rules! token {
//     (
//         #[from($kind:ident)]
//         $(#[$meta:meta])*
//         struct $name:ident;
//     ) => {
//         #[derive(Debug, Clone, PartialEq, Eq, Hash)]
//         $(#[$meta])*
//         pub struct $name(pub(super) SyntaxToken);

//         impl std::fmt::Display for $name {
//             fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//                 std::fmt::Display::fmt(self.syntax(), f)
//             }
//         }

//         impl AstToken for $name {
//             fn can_cast(kind: SyntaxKind) -> bool {
//                 $kind == kind
//             }

//             fn cast(from: SyntaxToken) -> Option<Self> {
//                 if from.kind() == $kind {
//                     Some(Self(from))
//                 } else {
//                     None
//                 }
//             }

//             fn syntax(&self) -> &SyntaxToken {
//                 &self.0
//             }
//         }
//     };
// }

// token! { #[from(TOKEN_WHITESPACE)] struct Whitespace; }

fn dir_tests<F>(dir: &str, get_actual: F)
where
    F: Fn(String) -> String,
{
    let base_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test_data", dir].iter().collect();
    let success_path = base_path.join("success");
    let error_path = base_path.join("error");

    let entries = success_path.read_dir().unwrap().chain(error_path.read_dir().unwrap());

    for entry in entries {
        let path = entry.unwrap().path();

        if path.extension() != Some(OsStr::new("nix")) {
            continue;
        }

        println!("testing: {}", path.display());

        let mut code = fs::read_to_string(&path).unwrap();
        if code.ends_with('\n') {
            code.truncate(code.len() - 1);
        }

        let actual = get_actual(code);
        expect_file![path.with_extension("expect")].assert_eq(&actual);
    }
}

#[test]
fn parser_dir_tests() {
    dir_tests("parser", |code| {
        let parse = Root::parse(&code);

        let mut actual = String::new();
        for error in parse.errors() {
            writeln!(actual, "error: {}", error).unwrap();
        }
        writeln!(actual, "{:#?}", parse.syntax()).unwrap();

        actual
    })
}

#[test]
fn type_tokenizer_dir_tests() {
    dir_tests("tokenizer", |code| {
        let mut actual = String::new();
        for (kind, str) in tokenize(&code) {
            writeln!(actual, "{:?}, \"{}\"", kind, str).unwrap();
        }
        actual
    });

    // let file = PathBuf::from("./test.nix");
    // let src = fs::read_to_string(&file).unwrap();

    // Example how to iterate through all comments
    //
    // let comments: Vec<String> = rnix::Root::parse(&src)
    //     .ok()
    //     .unwrap()
    //     .syntax()
    //     .children_with_tokens()
    //     .filter_map(|e| match e {
    //         rowan::NodeOrToken::Token(token) => match_ast! { match token {
    //             ast::Comment(it) => {
    //                 let mut actual = String::new();
    //                 for (kind, str) in tokenize(&it.text()) {
    //                     // println!("tnix token: {:?}, \"{}\"", kind, str);
    //                     writeln!(actual, "{:?}, \"{}\"", kind, str).unwrap();
    //                 }

    //                 Some(it.text().to_string())
    //             },
    //             _ => None,
    //         }},
    //         rowan::NodeOrToken::Node(_) => None,
    //     })
    //     .collect();
    // println!("{comments:?}");

    // Ok(())
}
