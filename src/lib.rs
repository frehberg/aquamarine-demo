//! A demo crate for [aquamarine](https://docs.rs/aquamarine)


/// This is a comment.
///
/// This is a diagram[^1]:
///
/// <script> const x = 1;</script>
///
/// [^1]: This is a footnote.
pub fn example_with_line_quote() {}


/// This is a comment.
///
/// This is a diagram[^1]:
/**  
  <script> const x = 1;</script>
*/
/// [^1]: This is a footnote.
pub fn example_with_block_comment() {}

#[aquamarine::aquamarine]
/// A function showcasing aquamarine defaults
///
/// This is a diagram[^1]:
/// 
/// ```mermaid
/// graph LR
///     s([Source]) --> a[[aquamarine]]
///     r[[rustdoc]] --> f([Docs w/ Mermaid!])
///     subgraph rustc[Rust Compiler]
///     a -. inject mermaid.js .-> r
///     end
/// ```
/// [^1]: This is a footnote.
pub fn example_fail_with_aquamarine() {}

