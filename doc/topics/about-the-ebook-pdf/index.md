# About the ebook PDF

The Rust Quick Guide ebook PDF is generated from the repository markdown files. The process uses custom book build tools, fonts thanks to Adobe, our open source tools, and the program `pandoc`.


## Book build tools

The book build tools are in the repository, in the directory `book/build`. The tools select all the documentation links, merge all the markdown files, then process everything into a PDF file.


## Book fonts

The book fonts are Source Serif Pro, Source Sans Pro, and Source Code Pro. The fonts are by Adobe and are free open source. 


## markdown-text-to-link-urls

<https://github.com/sixarm/markdown-text-to-link-urls>

This is a command-line parsing tool that we maintain. The tool reads markdown text, and outputs all markdown link URLs. We use this to parse the top-level file `README.md`, to get all the links. We filter these results to get the links to individual guidepost markdown files, then we merge all these files into one markdown file.


## pandoc-from-markdown-to-pdf

<https://github.com/sixarm/pandoc-from-markdown-to-pdf>

This is a command-line pandoc tool that we maintain. The tool providers our preferred pandoc settings in order to convert from an input markdown text file to an output PDF file. The tool adds a table of contents, loads our preferred fonts, configures source code syntax highlighting, sets the page size and margins, and more.
