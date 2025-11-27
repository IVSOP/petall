#import "@preview/charged-ieee:0.1.4": ieee

#show: ieee.with(
  title: [Evaluation of a Verifiable DBMS (Database Management System)],
  abstract: [Verifiable database management systems (verifiable DBMSs) aim to provide cryptographic guarantees about the correctness of query answers, allowing users to independently verify the integrity of the underlying data and computations. The PETALL project, developed in collaboration with INCM and awarded under the IN3+ innovation initiative, proposes a set of components that enable such verifiability through zero-knowledge techniques. This work presents an experimental evaluation of these components by integrating them into a prototype application designed for an energy community scenario. Our goal is to assess the suitability, limitations, and practical usability of the PETALL verifiable DBMS when applied to real-world application development. Through this case study, we analyze system behavior, data immutability constraints, adversarial considerations, and user interaction requirements. The results highlight both the potential and the challenges of adopting verifiable DBMS technologies, offering insights into their maturity and applicability in practice.

  ],
  authors: (
    (
      name: "XXXXXX",
      department: [Master's Student],
      organization: [University of Minho],
      location: [Braga, Portugal],
      email: "pgXXXXX@uminho.pt",
    ),
    (
      name: "Francisco Macedo Ferreira",
      department: [Master's Student],
      organization: [University of Minho],
      location: [Braga, Portugal],
      email: "pg55942@uminho.pt",
    ),
    (
      name: "XXXXXX",
      department: [Master's Student],
      organization: [University of Minho],
      location: [Braga, Portugal],
      email: "pgXXXXX@uminho.pt",
    ),
    (
      name: "XXXXXX",
      department: [Master's Student],
      organization: [University of Minho],
      location: [Braga, Portugal],
      email: "pgXXXXX@uminho.pt",
    ),
  ),
  index-terms: ("Verifiable DBMS", "Zero-Knowledge Proofs", "PETALL", "Energy Communities"),
  bibliography: bibliography("refs.bib", full: true),
)

= Introduction

A desenvolver um sistema de base de dados tal tal, como integrar isto num funcionamento seguro e user friendly

este projeto esta a desenvolver um sistema de base de dados que tem X caracteristicas

é preciso perceber como é que se consegue integrar esta sistema de base de dados num funcionamento que seja seguro por um lado e agradavel de utilizar por outro

== Case Study

usar como caso de uso, uma comunidade de energia

quem são os atores maliciosos?

usar zero knowledge para dizer que isto é justo

vamos fazer uma aplicação que permita um utilizador normal tirar partido disso

= Application

Aplicação , explicar interações (petall - comunidade) (utilizador - comunidade), modularidade das regras de pagamento (regra simplista que usa os dados que lá estão), explicar como é feita comunicação (falar das tecnologias) 

== Components

== Interactions between components

= Key Findings

remover utilizadores não dá direito, os dados tem que estar la sempre para provar coisas (dados imutaveis). o snapshot vai ter sempre um delay até algumas records poderem ser provados
