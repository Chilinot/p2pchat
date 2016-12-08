default: compile clean

OUTPUTNAME=LACPP-Project
MAINFILE=main.tex

# We need to compile the document twice for biber to work, since it relies on the generated output
compile:
	cat $(MAINFILE) | pdflatex -jobname=$(OUTPUTNAME)
	biber $(OUTPUTNAME)
	cat $(MAINFILE) | pdflatex -jobname=$(OUTPUTNAME)

# Remove all auto generated files
clean:
	rm -f *-blx.bib *.toc *.aux *.bbl *.bcf *.blg *.log *.run.xml

open: compile clean
	evince $(OUTPUTNAME).pdf

.PHONY: default compile clean open
