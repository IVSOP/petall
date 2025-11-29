#import "@preview/charged-ieee:0.1.4": ieee

#show: ieee.with(
  title: [Evaluation of a Verifiable DBMS (Database Management System)],
  abstract: [Verifiable database management systems (verifiable DBMSs) aim to provide cryptographic guarantees about the correctness of query answers, allowing users to independently verify the integrity of the underlying data and computations. The PeTall project, developed in collaboration with INCM and awarded under the IN3+ innovation initiative, proposes a set of components that enable such verifiability through zero-knowledge techniques. This work presents an experimental evaluation of these components by integrating them into a prototype application designed for an energy community scenario. Our goal is to assess the suitability, limitations, and practical usability of the PeTall verifiable DBMS when applied to real-world application development. Through this case study, we analyze system behavior, data immutability constraints, adversarial considerations, and user interaction requirements. The results highlight both the potential and the challenges of adopting verifiable DBMS technologies, offering insights into their maturity and applicability in practice.

  ],
  authors: (
    (
      name: "Ivan Ribeiro",
      department: [Master's Student],
      organization: [University of Minho],
      location: [Braga, Portugal],
      email: "pg55950@uminho.pt",
    ),
    (
      name: "Francisco Ferreira",
      department: [Master's Student],
      organization: [University of Minho],
      location: [Braga, Portugal],
      email: "pg55942@uminho.pt",
    ),
    (
      name: "Diogo Marques",
      department: [Master's Student],
      organization: [University of Minho],
      location: [Braga, Portugal],
      email: "pg55931@uminho.pt",
    ),
    (
      name: "Pedro Carvalho",
      department: [Master's Student],
      organization: [University of Minho],
      location: [Braga, Portugal],
      email: "pg55997@uminho.pt",
    ),
  ),
  index-terms: ("Verifiable DBMS", "Zero-Knowledge Proofs", "PeTall", "Energy Communities"),
  bibliography: bibliography("bibliography.yaml", full: true),
)

#let todo = text.with(fill: red)

= Introduction

//A desenvolver um sistema de base de dados tal tal, como integrar isto num funcionamento seguro e user friendly

//este projeto esta a desenvolver um sistema de base de dados que tem X caracteristicas

//é preciso perceber como é que se consegue integrar esta sistema de base de dados num funcionamento que seja seguro por um lado e agradavel de utilizar por outro

//FIXME: muito parecido com abstract? 
Ensuring correctness and integrity of data returned from a database is a fundamental requirement in applications whose trust, transparency, and auditability are crucial. Although traditional DBMSs support trusted administrators and secure infrastructures, they cannot cryptographically prove the correctness of query answers. Verifiable DBMSs aim to bridge this gap by enabling independent validation from users over query results, reducing the amount of trust required in system operators.

The PeTall project, developed in cooperation between research institutions and INCM, and distinguished with an IN3+ innovation award, investigates this paradigm through the development of components that enable support for query verifiability using zero-knowledge mechanisms. Instead of implementing a DBMS from scratch, the work that follows experimentally evaluates the PeTall system and assesses its readiness for real-world application development.

To structure this assessment, we define and implement a prototype using a relevant use case: an energy community, where several actors both contribute and consume energy, and require transparent distribution rules on the basis of immutable data. This setting provides realistic challenges for verifiable DBMS technology regarding adversarial behavior, fairness constraints, and the need for high-quality user interaction.

= Case Study

//usar como caso de uso, uma comunidade de energia

Residential adoption of solar photovoltaic systems has increased significantly over the past few years @statista_residential_pv_europe_2025. Although these households frequently produce surplus energy, a substantial portion of this energy is wasted due to limitations in storage or local consumption. Energy Communities address this inefficiency by enabling households to share excess production with other members, improving sustainability and reducing collective energy costs.

However, sharing energy fairly within a community is non-trivial. Different households exhibit distinct production and consumption patterns, and a naive distribution mechanism may favor certain members disproportionately. For instance, users who consistently produce more energy could repeatedly contribute more than they receive, resulting in systematic unfairness. The energy distribution must also be provable; otherwise, the energy providers themselves might fabricate or omit data to the user for their own financial gain, possibly controling when energy is shared or sold. These fairness concerns motivate the need for transparent and auditable rules governing energy allocation.

== Problem 

One straightforward solution to guaranteeing fairness is to make all production and consumption records publicly available. If every household can inspect every other household's energy history, then each member can verify whether distributions were performed correctly and whether they have received an equitable share.

While an effective solution, this approach introduces a new problem regarding the privacy of the users. Energy usage patterns may reveal highly sensitive information about the occupancy status of the household, such as when residents are home, asleep, at work, or away on vacation, or the type of devices that are being powered. Such information could be exploited by malicious actors, including burglars or other adversaries seeking to infer schedules, household routines, profile households based on estimated wealth or asset levels, or any other sensitive information. Therefore, any realistic system must balance auditability with strong privacy guarantees.

//TODO: quem são os atores maliciosos?

//usar zero knowledge para dizer que isto é justo

== Zero-Knowledge as a Solution

To reconcile fairness with privacy, the case study relies on zero-knowledge proofs (ZKPs). PeTall's verifiable DBMS produces cryptographic proofs over immutable snapshots of the database. These proofs allow each participant to verify the correctness of the computed energy distribution without learning the individual production or consumption values of other households.

For example, if a user contributed 10 kWh to the community, the system can produce a proof that they are entitled to receive at least 10 kWh at a later time, while not revealing to this user how much energy any other user contributed or consumed.

//vamos fazer uma aplicação que permita um utilizador normal tirar partido disso

= Application

// == Requirements

// === Community Application

// A frontend should be made available to the users of this service, so they can interact not only with the energy communities, but also have access to the ZKP systems. In this case study, the frontend was developed with certain requirements in mind, to show how a real application could interact with the verifiable DBMS while providing users with the desired functionalities:

// - Users must be able to register and login into the application.
// - Users must have a profile with information such as their name. They must be able to create and edit this information.
// - Energy records are provided, for a given household, at 15 minute intervals.
// - A single user might be a member of multiple energy communities.
// - Within an energy community, a user must be able to see his energy records, containing information about the involved energy and monetary value.
// - Within an energy community, a user must be able to see relevant statistics over his energy records for some time interval.


// == Access Control and Roles

// We defined three separate roles to enforce access control and ensure proper delegation of responsibilities within the system, with each role providing a distinct level of permissions, ranging from basic user access to full administrative control. This layered approach allows for decentralized community management while maintaining centralized oversight when necessary.

// === *User*
// This is the default role assigned to any new registered user. Users can view their own energy production and consumption records within communities they belong to. They have no administrative privileges and cannot manage community membership or settings.

// === *Admin*
// Admins have unrestricted access across the entire platform. They can create new communities, manage all existing communities regardless being assigned to them or not, and oversee the complete user base.


// === *Community Manager*
// A user chosen to oversee one or more communities. Managers can access the administrative interface for their assigned communities, being able to add or remove members and appoint other managers. Their permissions are exclusive to the communities they manage, ensuring decentralized administration without full sytem access.


#figure(
  table(
    columns: 4,
    align: (left, center, center, center),
    table.header(
      [*Action*], [*User*], [*Manager*], [*Admin*],
    ),
    [View own energy data], [Yes], [Yes], [Yes],
    [Access admin view], [No], [Yes], [Yes],
    [View managed communities], [No], [Own only], [All],
    [Create communities], [No], [No], [Yes],
    [Add/remove users], [No], [Own only], [All],
    [Add/remove managers], [No], [Own only], [All],
  ),
  caption: [Role-based access control permissions],
) <tbl:role-permissions>

//Aplicação , explicar interações (petall - comunidade) (utilizador - comunidade), modularidade das regras de pagamento (regra simplista que usa os dados que lá estão), explicar como é feita comunicação (falar das tecnologias) 

== Petall Application

// ...




= Application Parts

== Database

// DESCREVER QUE SE USOU POSTGRES
// Falar de SQLX e Docker?

The application uses PostgreSQL 16, deployed via Docker for simplified setup and portability. 

=== Schema

With this requirements in mind we decided on a final database schema which can be seen in @fig:db-schema

#figure(
  placement: top, // Se nao tiver isto só fica numa coluna
  scope: "parent", // same here
  align(left, image("imgs/drawdbfinal.png", width: 120%)),
  caption: [Database Schema],
) <fig:db-schema>

+ *User table*: stores registered users in the system with each having an id (unique), name, email and a isAdmin flag which determines if the user has admin priviligies or not. 

+ *Key table*: manages user auth credentials, allowing multiple auth provides like email and google. With this table we can allow a user to have multiple login methods associated with the same account. 

+ *Session table*: maintains active user sessions, each having an expiration date and a unique identifier allowing us to manage user sessions when necessary.

+ *Community table*: represents the energy communities in the system. 

+ *Community-user table*:  records which users belong to which communities, allowing users to belong to multiple communities and multiple users within a community.

+ *Community-manager table*: defines which users are managers of which communities.

+ *Energy record table*: is the primary table for energy distribution calculations and verification using zero-knowledge proofs. It stores energy records for each user in each community. Each record contains the users generated and consume energy (kWh), and consumer and seller prices. A new record is generated each 15 minutes to simulate an electric meter. //TODO: querem dizer isto?


== Community backend - ivsop

This part of the application uses Rust and Axum. this exposes the database in a usable format to the frontend.

this inserts new data for testing each 15 minutes (in the real world this would come from the electricity provider)

=== Endpoints
+ *POST \/user/login*: takes username and password and authenticates the user
e mais uns quantos


== Community frontend - pedro

Svelte @svelte + tailwind @tailwind + shadcn @shadcn. this allows the user to see energy records in a well formatted and useful manner. also allows for admins to create and manage communities.

// - The user logs in the application inserting the username and password or logging into Google OAuth...
// - The frontend sends a request to the backend with the username and password /login ...
// - The backend answers with a sesssion that the user will use in the subsequent requests /{community}...

The user can also click on a validate button for each energy record which will be described how it works in the next component

== Trusted-entity #todo[chico]

this application is also made in rust with axum. it is responsible for contacting the ZK service and return validated responses to the user.

when the user clicks on a validate button, the comunity frontend sends a POST /validate to this component signed with a JWT that is present in the community backend component. the JWT has the following form:

```json
{ user: blabla, record: blabla }
```

== Zero-Knowledge Component - chico

as the ZK database was not ready at the time of this assingment, we had to improvise and make a Proof of Concept of how the ZK database would work. this proof of concept is also made in Rust + Axum and just makes a request to the community backend asking for an energy record information. in the real world, a zero knowledge database would receive the snapshot daily and perform validations on top of it.

+ *GET /validate*: returns a proof of concept response with the energy record values and a randomized string for user presentation

= Interactions Between Components - digo

// 





//TODO: falar do JWT


= Key Findings - chico




- encontramos uma arquitetura suitable para este problema no Fig. (ref +ara figura do diagrama de sequencia).
- remover utilizadores não dá direito, os dados tem que estar la sempre para provar coisas (dados imutaveis). o snapshot vai ter sempre um delay até algumas records poderem ser provados
- dizer JWT teve de ser feito daquela forma manhosa?
