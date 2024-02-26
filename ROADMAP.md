# Roadmap And Goals

If you would like to see your ideas and suggestions getting implemented, please
feel free to make a PR to the roadmap, let's get the conversation started. Any
contributions to the code base are welcome!

## Immediate Term

### Conversion and Normalisation

There is a vast swathe of specialised security toolings such as DAST, SAST,
SCA, various scanners, audit and compliance checkers etc, each producing their
own data schemas and output formats. As the adoption of OSCAL keeps rising,
undoubtedly it will become one of the primary means of documenting and asserting
security controls. How are we going to convert existing data format to OSCAL
in a relevant and standardised way is one of the first set of problems that
need to be addressed urgently, e.g. ingesting data output such as Sarif, Snyk,
SonarQube, Jfrog XRay and produce risk related blocks for Assessment Results,
or how to parse Splunk output and build observation related blocks for Plan of
Action and Milestones? These points are yet to be answered and warrants the
following actions to be taken:

- Liaise with companies that build these toolings and open up discussions on
normalisation of data
- Obtain advice and feedback from OSCAL end users on what they like to be
implemented, find common ground amongst use cases that can be generalised and
perhaps one day to be standardised
- Build bespoke conversion and normalisation solutions for OSCAL end users and
seek public feedback. Please free free to reach out if your organisation have
such needs

### Library and CLI
- Support recursive uuid update for all data types(Currently only core types)
- Support generating full-skeleton empty model templates with placeholders
- Generate auditable logs for CLI runs
- Document signing and encryption, better file integrity checks

## Medium Term

### Continuously Building OSCAL
- Incorporate into CI/CD pipelines to enable collection of relevant data
- Start looking into annotation/DSL based instrumentation point insertion

### OSCAL Visualiser
- Build a web application capable of inspecting, generating and editing OSCAL
model files
- Support building task lists, memos, action monitors and external references

### Library and CLI
- Support XML format?(This is uncertain given the amount of effort required
and somewhat limited use cases)
- Build adaptor to various storage solutions for documentation indexing and
safeguarding

## Long Term Goals

### Automatic OSCAL Enforcement
Why use OSCAL at all if the specified controls are not verifiable and enforceable
automatically? This project will explore viable options to achieve this.
