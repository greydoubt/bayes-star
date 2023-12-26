# BAYES STAR

## Abstract
This **BAYES STAR** software package, I claim, unblocks **AGI**, by providing a "logical inference engine" that can do reasoning.

If you are interested in **AGI** you should be interested in this.

## Usage
For instructions on how to use the software see [USAGE.md](USAGE.md).

## The Quantified Bayesian Network
This software package introduces the **Quantified Bayesian Network** (**QBN**).
The QBN generalizes:
1. Traditional (generative) Bayesian Networks
    - **Bayesian Networks** are graphical models representing probabilistic relationships among variables. They use directed acyclic graphs to encode joint probability distributions, allowing for efficient reasoning and inference in complex systems. Bayesian Networks are widely used in various fields like machine learning, data analysis, and artificial intelligence for tasks like prediction, anomaly detection, and decision making.
    - Learn more:
        - [Bayesian Networks and their Applications](https://www.sciencedirect.com/topics/computer-science/bayesian-network)
2. First-Order Logic
    - **First-Order Logic** (FOL), also known as predicate logic or first-order predicate calculus, is a collection of formal systems used in mathematics, philosophy, linguistics, and computer science. It provides a framework for expressing statements with quantifiers and variables, allowing for the formulation of hypotheses about objects and their relationships. FOL is fundamental in formal systems, theorem proving, and is foundational in artificial intelligence for knowledge representation and reasoning.
    - Learn more:
        - [First-Order Logic: Basics](https://plato.stanford.edu/entries/logic-classical/)
        - [Understanding First-Order Logic](https://www.britannica.com/topic/formal-logic/Higher-order-and-modal-logic)

## How Does the QBN Avoid Hallucinations?
The QBN avoids hallucinations by:
1. using logic
2. understanding how to explain its argument
3. understands that there are things it does not know

How does it do this?
1. using logic -- the QBN generalizes (though in a complex way) first-order logic
2. using ideas from classical Bayesian Networks -- allows us to create a generative story based on "causality"

## Did you Literally Build AGI?
The QBN as I am presented it is trained on **artificial data**.

It will be AGI when the QBN is trained on **real web-scale data**.

Right now, the QBN only "thinks about" very simple worlds that I encoded by hand.
But, if we assume that the LLM has "world knowledge", then the only problem to get full AGI is to transfer the knowledge from the LLM to the QBN.

That, I claim would be full AGI. Right now, I repeat, the QBN is trained on "toy universes" that I made up programmatically.

## Is it Trivial to Transfer Knowledge from LLM to QBN?
No. This is not trivial. It will require that the LLM model be re-written to generate a **tree-structured** analysis of a sentence, mapping the **surface form** of the sentence to its **logical form**.

This **logical form** is **latent**--meaning we can't observe it, and neither can actual people (this is why misunderstandings arise).

So, the following new abilities need to be developed before "full AGI" exists:
1. parse to logical forms, which are:
    a. latent (not observed)
    b. structured (recursively tree-structured)
2. concretize the continuous knowledge of the LLM into the discrete knowledge of the QBN

## Does the QBN Help us Understand the LLM?
Yes, I believe so. The QBN uses "semantic roles", which might explain why the "key-value" nature of the attention mechanism can learn world knowledge:
that is, the **key-value** knowledge of the LLM is actually learning the **semantic role** knowledge of linguistics.

## Documentation

## Draft Latex Paper
I am putting all the "real" equations in an academic-style Latex-created [pdf document](paper/symbolic-logic-also-needed-DRAFT.pdf).
This is still in DRAFT STAGE.
As noted below, there is also a lot of multi-media content on my Twitter at [@coppola_ai](https://twitter.com/coppola_ai).
### Developing in Public
I am taking a "developing in public" vibe for this project.
I will ultimately publish a traditional paper to arXiv, if not elsewhere.

### Too Many Papers
But, I am agreeing with those who are saying there are "too many papers".
I have primarily a small core of ideas to communicate (e.g., the QBN).
I am happy to make any multi-media that helps people understand the idea.

### Tweet Threads
I have many tweet threads now discussing various aspects of this problem at [@coppola_ai](https://twitter.com/coppola_ai).
Actually, in some ways, this can be "better than a paper", or at least a great supplement to a paper, because:
* each idea is broken into little chunks
* there is multi-media
* you can start reading a chain and stop if you don't like the topic

I would like to organize the threads but that will come at the expense of just adding more content.

## References
The references file is a pretty good indication of the research that led to the QBN.
It is a lot of old-school 80's, 90's, 2000's (the "Baum" paper is from 1966).

This is based on my experience doing:
1. Master of Arts in Linguistics, focus on the "Meaning" of Natural Language, Simon Fraser University, 2007-2009
2. PhD in Computer Science, focus on Artificial Intelligence, The University of Edinburgh, 2009-2014
3. Experience in industry research and engineering (FAANG company)

See the [BibTeX File](paper/bibtex.bib) for references.

### Time-Stamping of Ideas on Bitcoin Chain
For time-stamping, I have put all of my work on the Bitcoin Blockchain ([address1](https://ordinals.hiro.so/inscriptions?a=bc1pjlpr5nzl6cmljtyz0a3gng98y3r5hs8z68gw55vg4ccjptvj9msq5gqrc5), [address2](https://ordinals.hiro.so/inscriptions?a=bc1pvd4selnseakwz5eljgj4d99mka25mk8pp3k7v7hc6uxw8txy6lgsf7lmtg)).
This provides a crypto-graphically secure time-stamping and immutable recording of each idea, whose accuracy is guarnateed by the value of the Bitcoin chain (almost $1 trillion).


Find me online at:
* twitter: [@coppola_ai](https://twitter.com/coppola_ai)

## License

This project is licensed under the ISC License - see the [LICENSE.txt](LICENSE.txt) file for details.
