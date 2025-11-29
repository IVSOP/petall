#import "@preview/charged-ieee:0.1.4": ieee
#import "@preview/chronos:0.2.1"

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
)

#let todo = text.with(fill: red)

= Introduction

//A desenvolver um sistema de base de dados tal tal, como integrar isto num funcionamento seguro e user friendly

//este projeto esta a desenvolver um sistema de base de dados que tem X caracteristicas

//é preciso perceber como é que se consegue integrar esta sistema de base de dados num funcionamento que seja seguro por um lado e agradavel de utilizar por outro

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


== Zero-Knowledge as a Solution

To reconcile fairness with privacy, the case study relies on zero-knowledge proofs (ZKPs). PeTall's verifiable DBMS produces cryptographic proofs over immutable snapshots of the database. These proofs allow each participant to verify the correctness of the computed energy distribution without learning the individual production or consumption values of other households.

For example, if a user contributed 10 kWh to the community, the system can produce a proof that they are entitled to receive at least 10 kWh at a later time, while not revealing to this user how much energy any other user contributed or consumed.

== Energy Distribution Rule

As mentioned above, energy sharing among community members must follow rules that minimize differences between consumption and production, but there are several ways to achieve this goal. With this in mind, each community has a calculation rule, defined at the time of its creation, from which the coefficients that dictate the allocation of energy to each user will be calculated.

It should be mentioned that this rule must necessarily be immutable, otherwise different logics would be applied in snapshots of the same energy community, which would not make any sense, as we would try to compensate through formula X, energy that was consumed according to formula Y.

//vamos fazer uma aplicação que permita um utilizador normal tirar partido disso

// = Application

//TODO: titulo a mais

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


// #figure(
//   table(
//     columns: 4,
//     align: (left, center, center, center),
//     table.header(
//       [*Action*], [*User*], [*Manager*], [*Admin*],
//     ),
//     [View own energy data], [Yes], [Yes], [Yes],
//     [Access admin view], [No], [Yes], [Yes],
//     [View managed communities], [No], [Own only], [All],
//     [Create communities], [No], [No], [Yes],
//     [Add/remove users], [No], [Own only], [All],
//     [Add/remove managers], [No], [Own only], [All],
//   ),
//   caption: [Role-based access control permissions],
// ) <tbl:role-permissions>

//Aplicação , explicar interações (petall - comunidade) (utilizador - comunidade), modularidade das regras de pagamento (regra simplista que usa os dados que lá estão), explicar como é feita comunicação (falar das tecnologias) 

// == Petall Application

// ...




= Application Parts

== Database


The application uses PostgreSQL 16 @postgres16, deployed via Docker @docker for simplified setup and portability. 

=== Schema

With this requirements in mind we decided on a final database schema which can be seen in @db-schemas.

+ *User* @tbl:user: stores registered users in the system with each having an id (unique), name, email and a isAdmin flag which determines if the user has admin priviligies or not. 

+ *Key* @tbl:key: manages user authentication credentials, allowing multiple auth provides like email and google. With this table we can allow a user to have multiple login methods associated with the same account. 

+ *Session* @tbl:session: maintains active user sessions, each having an expiration date and a unique identifier allowing us to manage user sessions when necessary.

+ *Community* @tbl:community: represents the energy communities in the system. 

+ *Community-user* @tbl:community_user:  records which users belong to which communities, allowing users to belong to multiple communities and multiple users within a community.

+ *Community-manager* @tbl:community_manager: defines which users are managers of which communities.

+ *Energy record* @tbl:energy_record: is the primary table for energy distribution calculations and verification using zero-knowledge proofs. It stores energy records for each user in each community. Each record contains the users generated and consume energy (kWh), and consumer and seller prices. A new record is generated each 15 minutes to simulate how an electric meter should work. 


== Community Backend

This component of the application was built with Rust @rust and Axum @axum, using the _sqlx_ crate @sqlx. It exposes endpoints that allow the frontend to retrieve and update data from the database. For example:

- `GET /community/{id}`: retrieves information on a specific community, like its name and image.
- `POST /community/{id}/energy`: retrieves energy records given a specific community and user. The user is extracted from the JWT, ensuring only a properly authenticated user can access his, and only his, data records.
- `POST /community/{id}/stats`: given a community and authenticated user like before, retrieves aggregated data over some period of time. For this purpose, it also receives a stats filter, specifying the desired start and end times, and the granularity of the data. This allows leveraging the database itself for aggregating data, instead of retrieving large batches of records to the frontend.
- Auth endpoints at `/auth/...`: provide an API for users to register, login, and interface with OAuth.

\

To improve our proof-of-concept functionality, two automatic seeding features where added:

+ When a user is added to a community, fake energy records using reasonable values are added up to 90 days before the current date. This ensures the frontend can immediately show data and statistics that need to aggregate data over some time period.
+ Every 15 minutes, a new fake energy record is added to each user, for each community that he belongs to. This allows simulating real conditions, where data is constantly being fed to the applications as the electricity meter periodically produces records.

// === Endpoints
// + *POST \/user/login*: takes username and password and authenticates the user
// e mais uns quantos

== Community Frontend 

We have used Svelte @svelte, Tailwind @tailwind and shadcn @shadcn to build our frontend. This allowed the user to see energy records in a pleasant and useful manner and also allows admins to create and manage communities.

- The user logs in the application inserting the username and password or logging into Google OAuth by sending a `POST` request to the backend with the credentials to the `/api/auth/login` endpoint
// - The frontend sends a request to the backend with the username and password /login ... #todo: nao estamos a repetir? 
- The backend validates the credentials and answers with a `session ID` that is stored as a cookie which will be used in all subsquent requests to authenticated endpoints such as `/community/{id}`.

- The frontend displays aggregated statistics about energy consumption and generation for each of the user's communities. The data comes pre-aggregated from the backend depending on the granularity the users chooses. For instance, if the user wants to see energy consumed and generated each month, the backend will group the data depending on its month and send it to the frontend, improving the system's performance and thus usability.

The user can also click on a validate button for each energy record. This functionality will be described in the next component.

== Trusted-Entity <trusted-entity>

To ensure that the zero-knowledge internals are operated by an independent and trusted party—such as, for example, Imprensa Nacional-Casa da Moeda—we delegate these responsibilities to a dedicated Trusted-Entity service. This service is solely responsible for validating energy records by communicating with the Zero-Knowledge (ZK) service, and shields the Community Backend from having direct access to or control over the ZK internals. This service was implemented in SvelteKit @svelte.

As shown in sequence diagram @fig:seq-dig3, when a community user clicks "Validate", the community frontend asks the Community Backend to mint a short-lived JWT via `GET /sign-energy-record-validation/{record}`. That token carries the user ID (taken from the user's authenticated session) plus the record ID, and it is signed with a private key known exclusively to the Community Backend; the matching public key is known to the Trusted-Entity service.

The frontend then calls the Trusted-Entity `POST /validate`, passing the JWT. The Trusted-Entity verifies the signature and expiry, extracts the user and record identifiers, and requests a ZK proof for that record. Once the ZK service returns the proof, the Trusted-Entity stores both the proof and record metadata in a user-scoped session (keyed by record ID) and issues a session cookie.

When the frontend later loads the validation page via `GET /validate?query={recordId}`, the Trusted-Entity reads the session cookie, retrieves the stored proof and record data, and renders the validation UI. Because the Community Backend only signs tokens for authenticated users and for records they own, no user can trigger validation for someone else's data. The session also allows that the user cannot share the proof with other users.

== Zero-Knowledge Component

Because the production Zero-Knowledge database was not ready during this assignment, we created a proof-of-concept service in Rust @rust and Axum @axum that mimics the responsibilities of the future ZK database. In the real deployment, this component would ingest the Community Backend's encrypted daily snapshots, and produce proofs over these snapshots.

For the POC, the ZK component exposes a single endpoint (`GET /validate`) that, when triggered by the Trusted-Entity service, fetches the requested energy record from the Community Backend (using a temporary endpoint `GET /energy-record-unauthenticated/{record}`) and returns both the raw values and a randomized string that stands in for the eventual zero-knowledge proof. This keeps the Trusted-Entity interface stable while allowing us to focus on the rest of the validation workflow.

= Interactions Between Components

To make the system truly functional, the various components must communicate with each other and share information, with the following flows being particularly noteworthy:

== Register User

As shown in sequence diagram @fig:seq-dig1, the user submits a registration form, sending a `POST /auth/register` request with username, email, and password in the body. The system creates the new user if it doesn't exist, generates and saves a `session ID`. Finally, the browser stores the `session ID`, automatically logging the user in. The entire process creates an account and authenticates the user in one flow.

== Remove User from Community
In the sequence shown in @fig:seq-dig2, the admin selects "remove user", triggering a `DELETE` request to `/admin/community/{community.id}/user`. The request is processed through the system, which checks the access permissions and deletes the `userCommunity` record from the database. Once confirmed, an `No Content` propagates back, and the `UI` updates to reflect that the user has been removed from the community.

== Validate energy record

@fig:seq-dig3 already described in @trusted-entity.

= Key Findings

Overall, the implementation demonstrates that the PeTall components can be assembled into a coherent and working validation flow, giving us confidence that the architecture moves into a functioning prototype, and the zero-knowledge is applicable in the real world.

The JWT-based handshake proved particularly effective: it keeps the Community Backend and the Trusted-Entity synchronized without duplicating authentication logic, while the signed tokens create a secure point-to-point channel that resists tampering even if the components are hosted by different organizations. 

Through this exercise we also found that the energy records need to be immutable. Because proofs are generated from that history and are re-validated at any time, records cannot be removed or rewritten; only softer relationships (such as user-to-community memberships) that no validation is performed on them may evolve independently without invalidating proof material. 

Finally, once the zero-knowledge functionality is fully integrated, there will inevitably be a delay between when a new energy record is created and when it becomes eligible for validation. This results from the need to synchronize data between the community database and the zero-knowledge database, which are separate systems. One practical solution is to periodically replicate data, for example through daily snapshots. Accordingly, the application should account for this lag and clearly communicate to users when new records will become available for verification.

= Conclusion

This project successfully delivered an architecture suited for a real-world application. Choosing Rust for the backend supplied a strongly typed, fail-safe, and fast runtime that can scale to large user loads. The surrounding ecosystem, Axum, SQLx, and other crates further streamlined development, making the development an enjoyable experience. On the frontend, Svelte combined with Tailwind and shadcn provided a consistent, user-friendly UI built from reusable components, and working in Svelte especially pleasant throughout the project. 

Regular meetings with the professors were instrumental for iterative refinement, ensuring the solution evolved alongside new insights. While the problem initially appeared straightforward, deeper investigation revealed non-trivial challenges that demanded deliberate solutions.

Future work includes integrating the production Zero-Knowledge service with data replication and, in practice, energy communities rely on distinct algorithms to distribute excess energy, so supporting provider-specific proof schemes will be essential.

#bibliography("bibliography.yaml", full: true)

#heading(numbering: none)[Appendix]

#[
 #set text(size: 0.5em)
  #figure(
    chronos.diagram({
      import chronos: *
      _par("User", display-name: [User])
      _par("CF", display-name: [Community Frontend])
      _par("CB", display-name: [Community Backend])
      _par("DB", display-name: [Postgres])
  
      _seq("User", "CF", comment: [Insert register \ parameters and \ submit], enable-dst: true)
      _seq(
        "CF",
        "CB",
        comment: [```
        POST /auth/register
        {
          username,
          email,
          password
        }
        ```],
        enable-dst: true,
      )
      _seq("CB", "DB", comment: [Insert a new row \ for that user], enable-dst: true)
      _seq("DB", "CB", comment: [Ok], disable-src: true)
      _seq("CB", "CB", comment: [Generate new `sessionId`\ for this session])
      _seq("CB", "DB", comment: [Save `sessionId` \ for that user], enable-dst: true)
      _seq("DB", "CB", comment: [Ok], disable-src: true)
      _seq(
        "CB",
        "CF",
        comment: ```json
        {
          uuid,
          name,
          emai,
          session_id,
        }
        ```,
        disable-src: true,
      )
      _seq("CF", "User", comment: [Set-Cookie `sessionId` \ and redirect to new page], disable-src: true)
    }),
    caption: [Sequence Diagram of user registration with simple auth],
  ) <fig:seq-dig1>

  #figure(
    chronos.diagram({
      import chronos: *
      _par("User", display-name: [Manager])
      _par("CF", display-name: [Community Frontend])
      _par("CB", display-name: [Community Backend])
      _par("DB", display-name: [Postgres])
  
      _seq("User", "CF", comment: [Clicks on the \ remove user button], enable-dst: true)
      _seq("CF", "CB", comment: ```
        DELETE 
        /admin
        /community
        /{id}/user
        {
          email
        }
        ```, enable-dst: true)
      _seq("CB", "CB", comment: [Extract JWT \ and validate \ `sessionId`])
      _seq("CB", "CB", comment: [Validate \ access \  permissions])
      _seq("CB", "DB", comment: [Remove user\ -community \ relationship], enable-dst: true)
      _seq("DB", "CB", comment: [Removed \ 1 row], disable-src: true)
      _seq("CB", "CF", comment: `204 No Content`, disable-src: true)
      _seq("CF", "User", comment: [Reload page], disable-src: true)
    }),
    caption: [Sequence Diagram for removing a user from community],
  ) <fig:seq-dig2>

  #figure(
    chronos.diagram({
      import chronos: *
      _par("User", display-name: [User])
      _par("CF", display-name: [Community Frontend])
      _par("CB", display-name: [Community Backend])
      _par("TE", display-name: [Trusted-Entity])
      _par("ZK", display-name: [ZK-Service])
  
      _seq("User", "CF", comment: [Clicks validate \ button for energy \ record `id`])
      _seq("CF", "CB", comment: [
        ```
        GET 
        /sign-energy-
        record-
        validation/{id}
        ```], enable-dst: true)
      _seq("CB", "CB", comment: [Sign a new `jwt` \ with the energy \ record id \ and user id])
      _seq("CB", "CF", comment: `OK { jwt }`, disable-src: true)
      _seq("CF", "TE", comment: ```
        POST /validate 
        { jwt }
        ```, enable-dst: true)
      _seq("TE", "TE", comment: [Validate and \ extract JWT])
      _seq("TE", "ZK", comment: ```
        GET 
        /validate
        /{record}
        ```, enable-dst: true)
      _seq(
        "ZK",
        "ZK",
        comment: [Fetch and \ compute \  proof #footnote[In the current implementation this is done by querying the Community Backend directly as the ZK database was not ready.]],
      )
      _seq("ZK", "TE", comment: ```
        Ok 
        { 
          proof, 
          record 
        }```, disable-src: true)
      _seq("TE", "TE", comment: [Create session for \ that user id and \ store `proof` \ and `record`])
      _seq("TE", "CF", comment: [Ok {query.id} and \ store session cookie], disable-src: true)
      _seq("CF", "TE", comment: [Open new window with \ `GET /validate?query={query.id}`], enable-dst: true)
      _seq("TE", "TE", comment: [Validate session \ from cookie])
      _seq("TE", "TE", comment: [Check session \ permission \ to view query])
      _seq("TE", "User", comment: [Show energy record proof], disable-src: true)
    }),
    caption: [Sequence Diagram for validate energy record],
  ) <fig:seq-dig3> 
]

== Community Database Schema <db-schemas>

#show table.cell.where(y: 0): set text(weight: "bold")

#set text(size: 0.7em)
#set par(justify: false)

#let db-table(name: ``, ..content) = figure(table(
  columns: 4,
  align: (x, y) => if y == 0 { center } else { left },
  fill: (x, y) => if y == 0 { gray.lighten(10%) } else if x == 0 { gray.lighten(50%) },
  table.header(

    [Field], [Type], [Constraint], [Description]
  ),
  ..content
), caption: [Database Table #name])

#db-table(name: `user`,
  `id`, `UUID`, [Primary Key], [The identifier of the user],
  `name`, `VARCHAR`, [Not Null], [The name of the user],
  `email`, `VARCHAR`, [Unique], [The email of the user],
  `isAdmin`, `BOOLEAN`, [Not Null], [Determines if the user is admin]
)<tbl:user>

#db-table(name: `key`,
  `provider`, [One of `email`, \ `github` or \ `google`], [Primary Key], [Authentication provider],
  `id`, `VARCHAR`, [Primary Key], [Identifier inside the provider namespace],
  `userId`, `UUID`, [Foreign Key \ `user.id`], [Owner of this auth credential],
  ```
  hashed
  Password
  ```, `VARCHAR`, [], [Hash for password-based providers (empty otherwise)]
)<tbl:key>

#db-table(name: `session`,
  `id`, `UUID`, [Primary Key], [Session identifier stored in cookies],
  `userId`, `UUID`, [Foreign Key `user.id`], [User associated with this session],
  `expiration`, `TIMESTAMPTZ`, [Not Null], [When the session becomes invalid]
) <tbl:session>

#db-table(name: `community`,
  `id`, `UUID`, [Primary Key], [Identifier of the community],
  `name`, `VARCHAR`, [Unique], [Name of the community],
  `description`, `TEXT`, [], [Long-form description for members],
  `image`, `VARCHAR`, [], [Optional URL of the community image]
) <tbl:community>

#db-table(name: `community_user`,
  `userId`, `UUID`, [Primary Key and \ Foreign Key \ `user.id`], [Member that belongs to the community],
  `communityId`, `UUID`, [Primary Key and \ Foreign Key \ `community.id`], [Community to which the user belongs]
) <tbl:community_user>

#db-table(name: `community_manager`,
  `userId`, `UUID`, [Primary Key and \ Foreign Key \ `user.id`], [Manager with elevated rights],
  `communityId`, `UUID`, [Primary Key and \ Foreign Key \ `community.id`], [Community which it \ refers to]
) <tbl:community_manager>

#db-table(name: `energy_record`,
  `id`, `UUID`, [Primary Key], [Identifier of the entry],
  `userId`, `UUID`, [Foreign Key \ `user.id`], [User associated \ with this record],
  `communityId`, `UUID`, [Foreign Key \ `community.id`], [Community context of the record],
  `generated`, `NUMERIC`, [], [Energy \ generated (kWh)],
  `consumed`, `NUMERIC`, [], [Energy \ consumed (kWh)],
  `consumerPrice`, `NUMERIC`, [], [Price paid by the consumer],
  `sellerPrice`, `NUMERIC`, [], [Price received by the seller],
  `start`, `TIMESTAMP`, [Not Null], [Start timestamp \ of the recorded \ interval (every \ 15 minutes)]
) <tbl:energy_record>
