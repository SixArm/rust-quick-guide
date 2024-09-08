# Makefile for the book
#
# If you get an error about XeLaTex, such as:
#
#     xelatex not found
#
# Then try adding it to your path, such as:
#
#     export PATH="$PATH:/Library/TeX/texbin/"
#

top = $(shell git rev-parse --show-toplevel)
key = $(shell basename ${top})

all: book.md book.pdf

.PHONY: clean
clean:
	@echo "make clean"
	rm -f book.md
	rm -f book.pdf

.PHONY: help
help:
	@echo "pandoc:"
	@which pandoc
	@pandoc --version
	@echo "pandoc-from-markdown-to-pdf:"
	@which pandoc-from-markdown-to-pdf

book.md:
	@echo "make book.md"
	book-binder book.yml book.md "${top}/README.md" "https://github.com/SixArm/rust-quick-guide/tree/main/projects"
	rsync -av book.md  "${top}/rust-quick-guide-1.0.0.md"

book.pdf:
	@echo "make book.pdf"
	pandoc-from-markdown-to-pdf book.md -o book.pdf
	rsync -av book.pdf "${top}/rust-quick-guide-1.0.0.pdf"
