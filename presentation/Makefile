default: compile clean

OUTPUTNAME=LACPP-Project_Presentation
MAINFILE=main.tex

# We need to compile the document twice for biber to work, since it relies on the generated output
compile:
	cat $(MAINFILE) | pdflatex -jobname=$(OUTPUTNAME)
	#biber $(OUTPUTNAME)
	#cat $(MAINFILE) | pdflatex -jobname=$(OUTPUTNAME)

# Remove all auto generated files
clean:
	rm -f $(OUTPUTNAME).nav $(OUTPUTNAME).out $(OUTPUTNAME).snm $(OUTPUTNAME)-blx.bib $(OUTPUTNAME).toc $(OUTPUTNAME).aux $(OUTPUTNAME).bbl $(OUTPUTNAME).bcf $(OUTPUTNAME).blg $(OUTPUTNAME).log $(OUTPUTNAME).run.xml

open: compile clean
	evince $(OUTPUTNAME).pdf

.PHONY: default compile clean open
