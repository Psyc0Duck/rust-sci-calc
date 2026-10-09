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
- Zufallstest (Fuzzing) des Parsers in den automatischen Tests.
- Dependabot aktualisiert Abhängigkeiten (mit 7 Tagen Wartezeit für neue Versionen).
- `cargo deny` prüft wöchentlich auf bekannte Lücken, erlaubte Lizenzen und Herkunft (nur crates.io).
- GitHub Actions sind auf feste Commit-Hashes gepinnt und laufen mit minimalen Rechten.
