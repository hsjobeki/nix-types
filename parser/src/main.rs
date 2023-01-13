extern crate rnix;
extern crate serde;
extern crate serde_json;
extern crate structopt;
use rnix::parser::{ASTKind, ASTNode, Arena, Data};
use rnix::tokenizer::Meta;
use rnix::tokenizer::Trivia;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::path::PathBuf;
use structopt::StructOpt;

/// Command line arguments for nixdoc
#[derive(Debug, StructOpt)]
#[structopt(name = "nixdoc", about = "Generate Docbook from Nix library functions")]
struct Options {
    /// Nix file to process.
    // #[structopt(short = "f", long = "file", parse(from_os_str))]
    // file: PathBuf,

    /// directory to process.
    #[structopt(short = "D", long = "dir", parse(from_os_str))]
    dir: PathBuf,
    // /// Name of the function category (e.g. 'strings', 'attrsets').
    // #[structopt(short = "c", long = "category")]
    // category: String,

    // /// Description of the function category.
    // #[structopt(short = "d", long = "description")]
    // description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManualEntry {
    pub id: String,
    pub name: String,
    pub fn_type: Option<String>,
}
#[derive(Debug)]
struct DocComment {
    /// Optional type annotation for the thing being documented.
    doc_type: Option<String>,
}

#[derive(Debug)]
struct DocItem {
    name: String,
    comment: DocComment,
}

/// Retrieve documentation comments. For now only multiline comments
/// starting with `@doc` are considered.
fn retrieve_doc_comment(allow_single_line: bool, meta: &Meta) -> Option<String> {
    for item in meta.leading.iter() {
        if let Trivia::Comment {
            multiline, content, ..
        } = item
        {
            if *multiline || allow_single_line {
                return Some(content.to_string());
            }
        }
    }
    return None;
}

/// Transforms an AST node into a `DocItem` if it has a leading
/// documentation comment.
fn retrieve_doc_item(node: &ASTNode) -> Option<DocItem> {
    // We are only interested in identifiers.
    if let Data::Ident(meta, name) = &node.data {
        let comment = retrieve_doc_comment(false, meta)?;

        return Some(DocItem {
            name: name.to_string(),
            comment: parse_doc_comment(&comment),
        });
    }
    return None;
}

fn parse_doc_comment(raw: &str) -> DocComment {
    enum ParseState {
        Doc,
        Type,
    }

    let mut doc_type = String::new();
    let mut state = ParseState::Doc;

    for line in raw.trim().lines() {
        let mut line = line.trim();

        if line.starts_with("Type:") {
            state = ParseState::Type;
            line = &line[5..]; // trim 'Type:'
        }

        match state {
            ParseState::Type => doc_type.push_str(line.trim()),
            ParseState::Doc => {
                
            }
        }
    }

    let f = |s: String| if s.is_empty() { None } else { Some(s.into()) };

    DocComment {
        doc_type: f(doc_type),
    }
}

/// Traverse a pattern argument, collecting its argument names.
/// Traverse a Nix lambda and collect the identifiers of arguments
/// until an unexpected AST node is encountered.
///
/// This will collect the argument names for curried functions in the
/// `a: b: c: ...`-style, but does not currently work with pattern
/// functions (`{ a, b, c }: ...`).
///
/// In the AST representation used by rnix, any lambda node has an
/// immediate child that is the identifier of its argument. The "body"
/// of the lambda is two steps to the right from that identifier, if
/// it is a lambda the function is curried and we can recurse.
/// Traverse the arena from a top-level SetEntry and collect, where
/// possible:
///
/// 1. The identifier of the set entry itself.
/// 2. The attached doc comment on the entry.
/// 3. The argument names of any curried functions (pattern functions
///    not yet supported).
fn collect_entry_information<'a>(arena: &Arena<'a>, entry_node: &ASTNode) -> Option<DocItem> {
    // The "root" of any attribute set entry is this `SetEntry` node.
    // It has an `Attribute` child, which in turn has the identifier
    // (on which the documentation comment is stored) as its child.
    let attr_node = &arena[entry_node.node.child?];
    let ident_node = &arena[attr_node.node.child?];

    // At this point we can retrieve the `DocItem` from the identifier
    // node - this already contains most of the information we are
    // interested in.
    let doc_item = retrieve_doc_item(ident_node)?;

    // From our entry we can walk two nodes to the right and check
    // whether we are dealing with a lambda. If so, we can start
    // collecting the function arguments - otherwise we're done.
    // let assign_node = &arena[attr_node.node.sibling?];
    // let content_node = &arena[assign_node.node.sibling?];
    Some(doc_item)
}

fn main() {
    let opts = Options::from_args();
    let paths = fs::read_dir(&opts.dir).unwrap();
    println!("{paths:?}");
    let mut data: Vec<ManualEntry> = vec![];
    for path in paths {
        let file_path = path.unwrap();
        let file_type = file_path.file_type().unwrap();
        let file = file_path.path();
        if file_type.is_file() && file.extension().unwrap() == "nix" {
            // sources.push(file);
            let src = fs::read_to_string(&file).unwrap();
            let nix = rnix::parse(&src).unwrap();
            let filename = file.file_stem().unwrap().to_str().unwrap();
            let parent = file.parent().unwrap().file_name().unwrap().to_str().unwrap();

            let entries: Vec<ManualEntry> = nix
                .arena
                .into_iter()
                .filter(|node| node.kind == ASTKind::SetEntry)
                .filter_map(|node| collect_entry_information(&nix.arena, node))
                .map(|d| ManualEntry {
                    id: format!("{}.{}.{}", parent, filename, d.name),
                    name: d.name,
                    fn_type: d.comment.doc_type,
                })
                .collect();
            data.extend(entries);
        }
    }
    let json_file = File::create("data.json").unwrap(); // (Error::from("Could not open or create json file"));
    ::serde_json::to_writer(&json_file, &data).unwrap();
}
