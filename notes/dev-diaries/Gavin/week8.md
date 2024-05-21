string length:32760000, nodes: 57215860

The ratio of nodes to chars is 1.78 for the biggest tree I can make before running out of memory.
It has a coverage of 32.7 megabases, which I think will be around half of the transcriptome.

Actually, the transcriptome is ... 496 megabases!? How?

genome: 797,256,690
transcriptome: 496,024,614

Ahh... I see

---

"The use of lower/upper case letters and N/n letters in genomes sequences is not completely standardised and you should always check the specification of the resource you are using.
Lower case letters are most commonly used to represent “soft-masked sequences”, a convention popularised by RepeatMasker, where interspersed repeats (which covers transposons, retrotransposons and processed pseudogenes) and low complexity sequences are marked with lower case letters. Note that larger repeats, such as sizable tandem repeats, segmental duplications, and whole gene duplications are not generally masked.
However, there are other uses for lower/upper case letters, for example, Ensembl have used upper/lower case letters to represent exonic and intronic sequences respectively.
N and n nucleotides may represent “hard masked sequences”, where interspersed repeats and low complexity sequences are replaced by Ns. But N/ns may alternatively represent ambiguous nucleotides, indeed this is the IUPAC specification."
https://bioinformatics.stackexchange.com/questions/225/uppercase-vs-lowercase-letters-in-reference-genome

---

I looked in the reference genome and there are blocks of Ns which signify unknown bases. Maybe to stitch together contigs? Also, it seems possible that case simply isn't representing coding/non-coding in our reference genome
Since coding bases tend to make up less that 10% of the genome. However, plants can have up to or greater than 30%. Maybe Mimosa pudica has a really consensed genome? Or maybe the non-coding parts are deduplicated in the genome by virtue of being repetitive and difficult to count, bringin up the ratio?

Anyhow, once everything is working, we can try matching reads with the transcriptome as it currently works, but it may be necessary to handle splices ourselves.