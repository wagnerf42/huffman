all: pres.html pres.pdf


pres.html: pres.md
	pandoc -t revealjs -s --slide-level=2 -o pres.html pres.md --toc -V theme=beige -V revealjs-url=./reveal.js
	
pres.pdf: pres.md
	pandoc -t beamer pres.md -o pres.pdf
