#import "@preview/charged-ieee:0.1.4": ieee

#show: ieee.with(
  title: [Evaluation of a Verifiable DBMS (Database Management System)],
  abstract: [Verifiable database management systems (verifiable DBMSs) aim to provide cryptographic guarantees about the correctness of query answers, allowing users to independently verify the integrity of the underlying data and computations. The PETALL project, developed in collaboration with INCM and awarded under the IN3+ innovation initiative, proposes a set of components that enable such verifiability through zero-knowledge techniques. This work presents an experimental evaluation of these components by integrating them into a prototype application designed for an energy community scenario. Our goal is to assess the suitability, limitations, and practical usability of the PETALL verifiable DBMS when applied to real-world application development. Through this case study, we analyze system behavior, data immutability constraints, adversarial considerations, and user interaction requirements. The results highlight both the potential and the challenges of adopting verifiable DBMS technologies, offering insights into their maturity and applicability in practice.

  ],
  authors: (
    (
      name: "Ivan Sérgio Rocha Ribeiro",
      department: [Master's Student],
      organization: [University of Minho],
      location: [Braga, Portugal],
      email: "pg55950@uminho.pt",
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

//A desenvolver um sistema de base de dados tal tal, como integrar isto num funcionamento seguro e user friendly

//este projeto esta a desenvolver um sistema de base de dados que tem X caracteristicas

//é preciso perceber como é que se consegue integrar esta sistema de base de dados num funcionamento que seja seguro por um lado e agradavel de utilizar por outro

//FIXME: muito parecido com abstract? 
Ensuring correctness and integrity of returned data from a database is a fundamental requirement in applications whose trust, transparency, and auditability are crucial. Although traditional DBMSs support trusted administrators and secure infrastructures, they cannot cryptographically prove the correctness of query answers. Verifiable DBMSs aim to bridge this gap by enabling independent validation from users over query results, reducing the amount of trust required in system operators.

The PETALL project, developed in cooperation between research institutions and INCM, and distinguished with an IN3+ innovation award, investigates this paradigm through the development of components that enable support for query verifiability using zero-knowledge mechanisms. Instead of implementing a DBMS from scratch, the work that follows experimentally evaluates the PETALL system and assesses its readiness for real-world application development.

To structure this assessment, we define and implement a prototype using a relevant use case: an energy community, where several actors both contribute and consume energy, and require transparent distribution rules on the basis of immutable data. This setting provides realistic challenges for verifiable DBMS technology regarding adversarial behavior, fairness constraints, and the need for high-quality user interaction.

= Case Study

//usar como caso de uso, uma comunidade de energia

Residential adoption of solar photovoltaic systems has increased significantly over the past few years @statista_residential_pv_europe_2025
. Although these households frequently produce surplus energy, a substantial portion of this energy is wasted due to limitations in storage or local consumption. Energy Communities address this inefficiency by enabling households to share excess production with other members, improving sustainability and reducing collective energy costs.

However, sharing energy fairly within a community is non-trivial. Different households exhibit distinct production and consumption patterns, and a naive distribution mechanism may favor certain members disproportionately. For instance, users who consistently produce more energy could repeatedly contribute more than they receive, resulting in systematic unfairness. The energy distribution must also be provable; otherwise, the energy providers themselves might fabricate or omit data to the user for their own financial gain, possibly controling when energy is shared or sold. These fairness concerns motivate the need for transparent and auditable rules governing energy allocation.

== Problem 

One straightforward solution to guaranteeing fairness is to make all production and consumption records publicly available. If every household can inspect every other household's energy history, then each member can verify whether distributions were performed correctly and whether they have received an equitable share.

While an effective solution, this approach introduces a new problem regarding the privacy of the users. Energy usage patterns may reveal highly sensitive information about the occupancy status of the household, such as when residents are home, asleep, at work, or away on vacation, or the type of devices that are being powered. Such information could be exploited by malicious actors, including burglars or other adversaries seeking to infer schedules, household routines, profile households based on estimated wealth or asset levels, or any other sensitive information. Therefore, any realistic system must balance auditability with strong privacy guarantees.

//TODO: quem são os atores maliciosos?

//usar zero knowledge para dizer que isto é justo

== Zero-Knowledge as a Solution

To reconcile fairness with privacy, the case study relies on zero-knowledge proofs (ZKPs). PETALL's verifiable DBMS produces cryptographic proofs over immutable snapshots of the database. These proofs allow each participant to verify the correctness of the computed energy distribution without learning the individual production or consumption values of other households.

For example, if a user contributed 10 kWh to the community, the system can produce a proof that they are entitled to receive at least 10 kWh at a later time, while not revealing to this user how much energy any other user contributed or consumed.

//vamos fazer uma aplicação que permita um utilizador normal tirar partido disso

= Application

//Aplicação , explicar interações (petall - comunidade) (utilizador - comunidade), modularidade das regras de pagamento (regra simplista que usa os dados que lá estão), explicar como é feita comunicação (falar das tecnologias) 

== Components

#figure(
  placement: top,
  scope: "parent",
  image("imgs/drawdb_final.svg", width: 100%),
  caption: [
    Database Schema
  ],
) <fig:db-schema>

== Interactions between components

// === Auth? em que se explica user vs admin vs super admin e o sistema de JWT

= Key Findings

//remover utilizadores não dá direito, os dados tem que estar la sempre para provar coisas (dados imutaveis). o snapshot vai ter sempre um delay até algumas records poderem ser provados
// dizer JWT teve de ser feito daquela forma manhosa?
