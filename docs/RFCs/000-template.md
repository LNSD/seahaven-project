---
title: RFC template
name: RFC template
slug: 000-template
status: draft
tags: meta
editor: "Lorenzo Delgado <lnsdev@proton.me>"
contributors: [ "Lorenzo Delgado <lnsdev@proton.me>" ]
---

## Abstract
This document defines the official RFC template for the Seahaven project.
It serves as a meta-RFC and describes the structure authors should follow when proposing features, 
improvements, or deprecations.

## Rationale
The Seahaven RFC process is inspired by the Rust RFC process.
It serves as a lightweight governance mechanism for evolving the Seahaven project in a structured, 
collaborative, and transparent way.
The goal is to capture the design rationale behind significant changes — including new features, 
deprecations, and process changes — before implementation begins.
This ensures alignment across contributors and provides historical context for future reference.
Anyone may submit a proposal, and discussions take place openly.
RFCs are intended to be living design documents, reviewed and iterated on collaboratively.
Once accepted, they act as a contract and reference for the implementation phase.

## Style
The RFC documents must be written in Markdown and SHOULD follow Semantic Line Breaks 
([https://sembr.org/](https://sembr.org/)) for better readability and version control.
To express normative requirements and expectations clearly, authors SHOULD use the keywords defined in 
[RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119), such as MUST, SHOULD, MAY, etc.
These terms convey the strength of recommendations and are crucial for consistently interpreting the RFC content.
All sections in the RFC must use H2 (`##`) or deeper heading levels since the title will be rendered as H1 (`#`) 
in the final markdown document.

## Implementation

### Template metadata
Each RFC must begin with a frontmatter block using YAML format.
This block provides structured metadata that helps identify, categorize, and manage the RFC.

| Field        | Description                                                                                                                    | Required? |
|:-------------|:-------------------------------------------------------------------------------------------------------------------------------|:---------:|
| title        | A human-readable title for the RFC.                                                                                            | Yes       |
| name         | A machine-friendly identifier, often the same as title.                                                                        | Yes       |
| slug         | A unique identifier in format `<rfc number>-<kebab-case rfc name>` (e.g., `001-feature-name`) used as the canonical reference. | Yes       |
| status       | The lifecycle state of the RFC. One of `draft`, `stable`, or `deprecated`. If omitted, it is considered `draft` by default.    | Yes       |
| tags         | Array of tags for categorization (e.g. `meta`, `cli`, `config`).                                                               | No        |
| editor       | The person coordinating the discussion is responsible for merging.                                                             | Yes       |
| contributors | List of authors or collaborators involved in writing or reviewing the RFC.                                                     | Yes       |

This metadata block ensures consistency across the RFC corpus and supports automated tooling for indexing, 
rendering, and filtering.

### RFC template document
This section provides the actual template that authors should use when creating new RFCs.
It includes all required sections and formatting guidelines, along with explanations of what should be included 
in each section.
The template is designed to ensure consistency across all RFCs while providing flexibility for different types 
of proposals.

```
---
title: <rfc-title>
name: <rfc-name>
slug: <rfc-id>-<rfc-name>
status: <status>
tags: [ <tag1>, <tag2>, ... ]
editor: "<editor name> <editor email>"
contributors: [ "<contributor name> <contributor email>" ]
---

## Abstract
A concise summary (typically 2-3 sentences) that captures the essence of the RFC proposal.
The abstract should clearly state what is being proposed and why it matters,
helping readers quickly understand the scope and significance of the change.
It serves as a high-level overview that enables stakeholders to determine
if they need to read the full document.

## Background/Rationale/Motivation
This section provides crucial context and justification for the proposed change or feature.

It SHOULD:

1. Describe the current state and any relevant limitations or pain points
2. Explain why the proposed change is necessary or beneficial
3. Outline the key problems or challenges being addressed
4. Provide supporting evidence or examples where applicable
5. Reference any related work or existing solutions

The background should be comprehensive enough to help readers understand the motivation
behind the proposal while remaining focused on the specific problem space.
This section forms the foundation for the rest of the RFC by establishing the need
for the proposed changes.

## Design/Explanation
This section SHOULD explain how the proposed change works, how it affects the existing system, 
and any relevant design decisions.
It MAY include diagrams, pseudocode, or high-level logic when necessary.

## Proposed Implementation (optional)
This section provides concrete implementation details and guidance for developers who will work 
on the proposed changes.

It SHOULD include:

1. Technical specifications and requirements
2. Code examples or pseudocode where relevant
3. API changes or additions
4. Database schema modifications if applicable
5. Configuration changes
6. Dependencies and version requirements
7. Migration steps or upgrade procedures
8. Testing requirements and strategies

The implementation suggestions should be detailed enough to guide development while remaining flexible 
enough to accommodate different technical approaches.

Authors SHOULD consider including:

- Performance considerations
- Backward compatibility requirements
- Error handling strategies
- Logging and monitoring recommendations
- Documentation requirements

This section is optional but highly recommended for standard track RFCs to ensure smooth implementation 
of the proposed changes.

## <Section title>
Extra optional sections that MAY be relevant to further explain the RFC proposal.
These sections should be added as needed to provide additional context, clarification, or details 
that don't fit naturally into the standard sections.

Common examples include:

- **Alternatives Considered**: Discussion of other approaches that were evaluated and why they were rejected
- **Impact Analysis**: Assessment of how the proposal affects different parts of the system or user workflows
- **Migration Strategy**: Detailed steps for transitioning from the current state to the proposed solution
- **Future Considerations**: Potential future enhancements or related features that might be built on top of this proposal
- **User Experience**: Details about how the proposal affects end users and their interactions with the system
- **Performance Impact**: Analysis of how the proposal affects system performance, resource usage, or scalability
- **Compatibility**: Discussion of how the proposal interacts with existing features or external systems
- **Testing Strategy**: Specific approaches for testing the proposed changes
- **Documentation Requirements**: Details about what documentation needs to be created or updated

Each additional section should follow the same formatting guidelines as the standard sections and should be 
clearly labeled with an H2 (`##`) heading.

## Security/Privacy Considerations
This section MUST be included in standard track RFCs with `stable` status.
This section SHOULD be included in standard track RFCs with `draft` status.
This section MAY be included in informational RFCs regardless of status.
If no security or privacy considerations exist, this section MUST explicitly state that fact.

Authors MUST consider and describe any security or privacy implications of the proposed change.
This section MUST address, but is not limited to:

- Data handling practices (e.g., sensitive information exposure)
- Attack surfaces introduced or expanded
- Access control and authorization considerations
- Any changes to trust boundaries or data flow

If there are no expected security or privacy concerns, this section MUST explicitly state so 
(e.g., "There are no known security or privacy considerations associated with this proposal.").

This section MAY include additional relevant information, such as explanations for why there are no 
security considerations for the respective document.

## Copyright
Copyright and related rights waived [via CC0](https://creativecommons.org/publicdomain/zero/1.0/).

## References
References SHOULD be organized into two distinct categories: normative and informative. 
This categorization helps readers understand which references are essential for implementation 
(normative) versus those that provide additional context or background information (informative). 
The distinction between normative and informative references follows the conventions established
 in [RFC-3967](https://datatracker.ietf.org/doc/html/rfc3967).
```

## Recommendations
When writing RFCs, consider the following guidelines:

1. **Be Clear and Concise**
   - Use simple, direct language
   - Break complex ideas into digestible sections
   - Include concrete examples where helpful

2. **Focus on the Problem**
   - Clearly state the problem before proposing solutions
   - Provide context and real-world scenarios
   - Explain why the current state is insufficient

3. **Consider Your Audience**
   - Write for both technical and non-technical readers
   - Define technical terms and acronyms
   - Include diagrams for complex concepts

4. **Encourage Discussion**
   - Be open to feedback and alternative viewpoints
   - Acknowledge potential trade-offs
   - Consider and document different perspectives

5. **Maintain Professional Tone**
   - Be respectful and constructive
   - Focus on technical merits rather than personal opinions
   - Use data and evidence to support claims

## Security/Privacy Considerations
There are no known security or privacy considerations associated with this proposal.

## Copyright
This template is based on the [VAC RFC Template](https://rfc.vac.dev/vac/template).

Copyright and related rights waived [via CC0](https://creativecommons.org/publicdomain/zero/1.0/).

## References

### Normative References
- [RFC2119] Bradner, S., "Key words for use in RFCs to Indicate Requirement Levels", 
  BCP 14, RFC 2119, DOI 10.17487/RFC2119, March 1997, 
  <https://www.rfc-editor.org/info/rfc2119>.

### Informative References
- [SEMBR] "Semantic Line Breaks", <https://sembr.org/>
- [RFC3967] "Clarifying when Standards Track Documents may Refer Normatively to Documents at a Lower Level", 
  RFC 3967, DOI 10.17487/RFC3967, December 2004,
  <https://datatracker.ietf.org/doc/html/rfc3967>
