# Sicherheit / Security

## Sicherheitslücke melden

Bitte melde Sicherheitslücken **nicht** als öffentliches Issue, sondern vertraulich über
[Security advisories](https://github.com/Psyc0Duck/rust-sci-calc/security/advisories/new)
("Report a vulnerability").

Please report vulnerabilities privately via the link above, not as a public issue.

## Unterstützte Versionen

Sicherheitsupdates gibt es nur für die jeweils neueste Version.

## Maßnahmen im Projekt

- Kein `unsafe` im eigenen Code (`#![forbid(unsafe_code)]`).
- Eigener Ausdrucks-Parser mit Grenzen für Eingabelänge und Verschachtelungstiefe.
- Dependabot aktualisiert Abhängigkeiten, `cargo audit` prüft sie wöchentlich auf bekannte Lücken.
