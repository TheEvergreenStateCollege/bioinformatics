I spent quite a bit of time this week continuing to research. I think we need to understand this problem before we can hope to tackle in, and the more I look into it the more I realize how complicated it is.

None of us are biologists and it will be easy for us to make mistakes based on a lack of understanding of biology. Some parts of sequencing like shortest common substring are biology agnostic, while others like splice-awareness are not. Luckily for me, my closest friend at Evergreen and my new roommate are both biologists and I spent hours this week getting them to explain genetics and molecular biology to me.

I also continued to watch the video series, read the paper on Eulerian walks, and did some other research. I came to some conclusions which I shared with the team on Friday. Notably, that we should start with read-alignment, which is easier than assembly and more practical with RNA data.

Dominic is looking into suffix-trees which look like they will be useful for read-alignment. The others are researching read-alignment. Kassidy is working on a general graph implementation in Rust as a foundation for graph based algorithms.

On Saturday I made a module for reading and parsing FASTA files in Rust. I deleted the Java-plants folder since it was doing the same thing. I got hung up on not being able to return a String along with references to it, but it seems to be impossible (rc can do it maybe). In the end I made two functions, one which returns a string and one which borrows it and returns structs containing slices of it.