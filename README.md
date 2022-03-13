[![crates.io](https://img.shields.io/crates/v/latex_parser.svg)][`latex_parser`]
[![crates.io](https://img.shields.io/crates/d/latex_parser.svg)][`latex_parser`]

# LaTeX parser

LaTeX is a language for typesetting documents, especially scientific papers, and a document preparation system.

# Example of .tex code

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

## Output

![tex output](https://i.imgur.com/xptzo3h.jpg)

[`latex_parser`]: https://crates.io/crates/latex_parser
[what is latex]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F

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
