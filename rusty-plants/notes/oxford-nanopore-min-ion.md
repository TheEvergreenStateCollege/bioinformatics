I translated the google docs file containing research someone did to markdown to include it in the notes

[https://nanoporetech.com/data-analysis?tab[tab]=command-line_tools](https://nanoporetech.com/data-analysis?tab[tab]=command-line_tools) 

[https://github.com/nanoporetech/minknow_api](https://github.com/nanoporetech/minknow_api) 

[https://hasindu2008.github.io/slow5specs/fast5_demystified.pdf](https://hasindu2008.github.io/slow5specs/fast5_demystified.pdf) 

[Oxford Nanopore MinION Sequencing and Genome Assembly - PMC](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5093776/)



* FAST5 files have a hierarchical structure, meaning that they can store both the metadata associated with a read, along with the events (such as aggregated bulk current measurements) pre-processed by the sequencing device. Each read is produced by one of the MinION’s 512 channels, and the metadata associated with each read are stored in a unique FAST5 file. To ensure the unique identity of each read name, a combination of information such as experiment name and batch, channel, as well as file numbers is used.
* [poRe: an R package for the visualization and analysis of nanopore sequencing data - PMC](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC4271141/)
    * R is a good language, used it in an ecology program \

    * Further info from Dom: R is a language specific to scientists and statisticians. It's slowly losing favor for python.  
* 

[The Oxford Nanopore MinION: delivery of nanopore sequencing to the genomics community ](https://genomebiology.biomedcentral.com/articles/10.1186/s13059-016-1103-0)



* RNA expression analysis is most often performed by NGS sequencing of cDNA copies. A drawback of this strategy is that the reads _are relatively short, thus requiring assembly of cDNA_ _reads_ _into full-length transcripts. _
        * This is an issue for the accurate characterization of RNA splice isoforms because there is often insufficient information to _deconvolute the different transcripts properly._
* In addition to official ONT tools (for example, ont_fast5_api software for format conversion between single-fast5 and multi-fast5 and data compression/decompression), several third-party software packages have been developed for quality control, format conversion (for example, NanoR for generating fastq files from fast5 files containing sequence information), data exploration and visualization of the raw ONT data (for example, Poretools, NanoPack and PyPore) and for after base-calling data analyses (for example, AlignQC and BulkVis) 
    * _Base calling, which decodes the current signal to the nucleotide sequence, is critical for data accuracy and detection of base modification_
* Metrichor cloud, DeepNano base-calling, Guppy bsse-calling, MarginAlign corrector, Nanopolish polishing algorithm, PoreSeq polishing, GraphMap structural variant detection…   
* 

[https://www.nature.com/articles/s41598-018-29334-5](https://www.nature.com/articles/s41598-018-29334-5) 

[Nanopore sequencing technology, bioinformatics and applications | Nature Biotechnology](https://www.nature.com/articles/s41587-021-01108-x) 



* Several other DNA modification detection tools followed, including Nanopolish (5mC), signalAlign (5mC, 5-hydroxymethylcytosine (5hmC) and 6mA), mCaller (5mC and 6mA), DeepMod (5mC and 6mA), DeepSignal (5mC and 6mA) and NanoMod (5mC and 6mA). Nanpolish, Megalodon and DeepSignal were recently benchmarked and confirmed to have high accuracy
* **The possibility of directly detecting _N_6-methyladenosine (m6A) <span style="text-decoration:underline;">modifications</span> in <span style="text-decoration:underline;">RNA molecules</span> was demonstrated using PacBio in 2012 (ref. [82](https://www.nature.com/articles/s41587-021-01108-x#ref-CR82)), although few follow-up applications were published. Recently, <span style="text-decoration:underline;">ONT direct RNA sequencing </span>has yielded robust data of reasonable quality, and several pilot studies have detected bulk-level RNA modifications by examining either <span style="text-decoration:underline;">error distribution profiles</span> (for example, EpiNano (m6A)[73](https://www.nature.com/articles/s41587-021-01108-x#ref-CR73) and ELIGOS (m6A and 5-methoxyuridine (5moU))[83](https://www.nature.com/articles/s41587-021-01108-x#ref-CR83)) or <span style="text-decoration:underline;">current signals </span>(for example, Tombo extension (m6A and m5C)[74](https://www.nature.com/articles/s41587-021-01108-x#ref-CR74) and MINES (m6A)[84](https://www.nature.com/articles/s41587-021-01108-x#ref-CR84)). _However, detection of RNA modifications with single-nucleotide resolution at the single-molecule level has yet to be demonstrated._**

[https://www.bioinformatics.uni-muenster.de/home/presentations/Software_overview.pdf](https://www.bioinformatics.uni-muenster.de/home/presentations/Software_overview.pdf) 

[Computational Biology @ Comenius University in Bratislava: DeepNano](https://compbio.fmph.uniba.sk/deepnano/)

[Analysis pipelines for nanopore sequence data](https://nanoporetech.com/resource-centre/analysis-pipelines-nanopore-sequence-data) (video from oxford site)

Mothur - [https://mothur.org/](https://mothur.org/) (“_The goal of mothur is to have a single resource to analyze molecular data that is used by microbial ecologists”_)

HDF5, HDF Group - [https://portal.hdfgroup.org/hdf5/](https://portal.hdfgroup.org/hdf5/)  (“_Our non-profit mission is to ensure efficient and equitable access to science and engineering data_..._data software library and file format to manage, process, and store your heterogeneous data_.”)

Guppy -  [https://timkahlke.github.io/LongRead_tutorials/BS_G.html](https://timkahlke.github.io/LongRead_tutorials/BS_G.html) 

Graphmap  - [https://github.com/isovic/graphmap](https://github.com/isovic/graphmap)

NanoPolish - [https://github.com/jts/nanopolish](https://github.com/jts/nanopolish) 

Links also on Discord:

[https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7799972/](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7799972/)

[https://darlinglab.org/mauve/user-guide/viewer.html](https://darlinglab.org/mauve/user-guide/viewer.html) 

[https://www.pnas.org/doi/10.1073/pnas.171285098](https://www.pnas.org/doi/10.1073/pnas.171285098)

…

Mimosa pudica nbci search results 

[https://www.ncbi.nlm.nih.gov/sra/?term=txid76306%5Borganism%3Aexp%5D+AND+biomol_dna%5Bprop%5D](https://www.ncbi.nlm.nih.gov/sra/?term=txid76306%5Borganism%3Aexp%5D+AND+biomol_dna%5Bprop%5D) 

…

[FASTA - Sequence Similarity Search](https://www.genome.jp/tools/fasta/) 

Other stuff:

[Dynamic Pooling Improves Nanopore Base Calling Accuracy](https://pubmed.ncbi.nlm.nih.gov/34784283/) (Osprey, Heron) 

[Library preparation and Single Molecule Real-Time (PacBio) sequencing](https://bio-protocol.org/exchange/minidetail?type=30&id=8698092) ([https://www.ncbi.nlm.nih.gov/sra/SRX15429257[accn](https://www.ncbi.nlm.nih.gov/sra/SRX15429257[accn)]) 

  

[Chapter 9 Bioinformatic Analysis of Nanopore Data](https://www.worldscientific.com/doi/pdf/10.1142/9789813270619_0009)

[https://training.galaxyproject.org/training-material/topics/assembly/tutorials/mrsa-nanopore/tutorial.html](https://training.galaxyproject.org/training-material/topics/assembly/tutorials/mrsa-nanopore/tutorial.html)  

[https://bio.tools/](https://bio.tools/) 


# Java FastQC

Wooh

[RNA sequencing analysis pipeline using STAR, RSEM, HISAT2 or Salmon with gene/isoform cou...](https://nf-co.re/rnaseq/3.12.0) 

[Fast QC Fork ](https://github.com/wagorndyl/FastQC)

[https://www.bioinformatics.babraham.ac.uk/projects/fastqc/](https://www.bioinformatics.babraham.ac.uk/projects/fastqc/) 

[https://broadinstitute.github.io/picard/](https://broadinstitute.github.io/picard/) 

[What are SAM & BAM Files? | ZYMO RESEARCH](https://www.zymoresearch.com/blogs/blog/what-are-sam-and-bam-files) 

[https://labs.epi2me.io/notebooks/Introduction_to_fastq_file.html](https://labs.epi2me.io/notebooks/Introduction_to_fastq_file.html)


    _line 1: Sequence ID and Sequence description_


    _line 2: Sequence line e.g. ATCGs_


    _line 3: plus symbol (can additionally have description here)_


    _line 4: Sequence line qualities_

(Space delineated)

>58812e1e-0fb0-4626-93d7-51e8e87c6969 runid=9d742d72b6f5d334c2d0d388f2eb1da13decd9a6 

sampleid=Plant_Memory_RNA_1 

read=4611 

ch=266 

start_time=2023-05-19T01:16:37Z model_version_id=2020-09-07_rna_r9.4.1_minion_256_8f8fc47b

^^^Switch statement, first bit same, other bits switch cases^^^ 

[Keys : data] (tuples) 

“Delineated” by spaces, keys by equals

Lines | spaces | data within space delineated 

While loop for lines until end of screen

Edge case: metadata is missing actual data 

If-else: need to know content of line

*LARGE DATA* 

= turn to String

Next steps (unlikely): Memory, multiple files, thread file reading(?)

<span style="text-decoration:underline;">FastQ -> …data format -> MATH </span>

<span style="text-decoration:underline;">→Create graph</span>

[https://biojava.org/wiki/BioJava:CookBook4.0/](https://biojava.org/wiki/BioJava:CookBook4.0/) 

[https://biojava.org/wiki/BioJava%3ACookBook3%3AFASTQ](https://biojava.org/wiki/BioJava%3ACookBook3%3AFASTQ) 

[https://biohpc.cornell.edu/doc/RNA-Seq-2019-exercise1.pdf](https://biohpc.cornell.edu/doc/RNA-Seq-2019-exercise1.pdf) 

[https://bioinformatics.ccr.cancer.gov/docs/b4b/Module2_RNA_Sequencing/Lesson10/](https://bioinformatics.ccr.cancer.gov/docs/b4b/Module2_RNA_Sequencing/Lesson10/) 

++—++

Biology resources completely just for fun: [https://ucdavis-bioinformatics-training.github.io/2020-mRNA_Seq_Workshop/](https://ucdavis-bioinformatics-training.github.io/2020-mRNA_Seq_Workshop/) 



??





[https://www.phind.com/search?cache=ysyiia3rbty10d0v3f8pqz1n](https://www.phind.com/search?cache=ysyiia3rbty10d0v3f8pqz1n) 

(Ignore me trying to get the AI to tell me if it did something randomly or on purpose despite already knowing it doesn’t “understand” a word it’s saying)
