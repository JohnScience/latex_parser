[![crates.io](https://img.shields.io/crates/v/latex_parser.svg)][`latex_parser`]
[![crates.io](https://img.shields.io/crates/d/latex_parser.svg)][`latex_parser`]

# LaTeX parser

## What is LaTeX?

LaTeX is a language for typesetting documents, especially scientific papers, and a document preparation system.

## Example of .tex code

```tex
% ...
\subsection*{H}
	\glossaryentry{hadamard_product}{Hadamard product}
	\begin{adjustwidth}{1em}{}
		\textbf{Field of study}: \textit{Mathematics. Linear Algebra. Matrix theory.} \\
		\textbf{Distinct meanings in other fields of study}: \textit{unspecified.} \\
		\textbf{Definitions}:
		\begin{adjustwidth}{1em}{} \leavevmode
			\begin{framed}
				For two \hyperlink{matrix}{\textit{matrices}} $A$ and $B$ of the same \hyperlink{dimension_of_matrix}{\textit{dimension}} $m \times n$, the \beingdefined{Hadamard product} $A \circ B$ (or $A \odot B$) is a \hyperlink{matrix}{\textit{matrix}} of the same \hyperlink{dimension_of_matrix}{\textit{dimension}} as the operands, with elements given by
				\begin{equation*}
					(A \circ B)_{ij} = (A \odot B)_{ij} = (A)_{ij}(B)_{ij}.
				\end{equation*}
				
				Source: \cite{wiki_hadamard_product_matrices}.
			\end{framed}
			\begin{framed}
				Let $A$ and $B$ be $m \times n$ \hyperlink{matrix}{\textit{matrices}} with entries in $C$. The \beingdefined{Hadamard product} is defined by $[A \circ B]_{ij}=[A]_{ij}[B]_{ij}$ for all $1 \leq i \leq m$, $1 \leq j \leq n$. \\ \vspace{1em}
				
				Source: \cite{emillion}.
			\end{framed}
		\end{adjustwidth}
	\end{adjustwidth} \vspace{1em}
% ...
```

### Output

![tex output](https://i.imgur.com/xptzo3h.jpg)

## Resources on LaTeX

* [LaTeX documentation on Overleaf](https://www.overleaf.com/learn)

# How [`latex_parser`] works

First and foremost, [`latex_parser`] is a [`nom`]-based parser.

## About [`latex_parser`] as a [`nom`]-based parser

According to [`nom`]'s documentation,

> nom is a [parser combinators][combinators] library written in Rust. Its goal is to provide tools to build safe parsers without compromising the speed or memory consumption. To that end, it uses extensively Rust's strong typing and memory safety to produce fast and correct parsers, and provides functions, macros and traits to abstract most of the error prone plumbing.

### [`nom`]'s documentation

* [docs.rs reference documentation](https://docs.rs/nom/latest/nom/)
* [Various design documents and tutorials](https://github.com/Geal/nom/tree/main/doc)
* [List of combinators and their behaviour][combinators]

## Design choices in [`latex_parser`]

To be compatible with [`nom`], [`Parse`] - the main trait of [`latex_parser`] - contains the method [`parse`] returning [`nom::IResult`]. This tight coupling makes `<T<'a> as Parse<'a>>::parse` [^1] an acceptable argument for [`nom`] [combinators], thereby simplifying the development of [`latex_parser`].

# Related crates

* [`include_display_mode_tex`] - a library for embedding TeX formulae in documentation.

[^1]: `<Type as Trait>::method` is so-called [Fully-Qualified Syntax](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name). Since all types in the [`latex_parser`] don't copy the contents of the parsed [strings](https://doc.rust-lang.org/rust-by-example/std/str.html), the references in the structure can't outlive the referenced [string slices](https://doc.rust-lang.org/core/primitive.str.html). So `'a` is the [lifetime](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-struct-definitions) of the stored references.

[`latex_parser`]: https://crates.io/crates/latex_parser
[what is latex]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
[`include_display_mode_tex`]: https://crates.io/crates/include_display_mode_tex
[`nom`]: https://crates.io/crates/nom
[`Parse`]: https://docs.rs/nom/latest/latex_parser/trait.Parse.html
[`parse`]: https://docs.rs/nom/latest/latex_parser/trait.Parse.html#tymethod.parse
[`nom::IResult`]: https://docs.rs/nom/latest/nom/type.IResult.html
[combinators]: https://github.com/Geal/nom/blob/main/doc/choosing_a_combinator.md

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
