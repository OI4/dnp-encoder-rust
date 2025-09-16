# DESIGN

## Ziel
Robuste, performante und spec-konforme Implementierung des OI4 / DIN SPEC 91406 DNP Maskierungsverfahrens.

## Regeln (Kurzfassung)
- Unreserved: `A–Z a–z 0–9 - . _ ~` unverändert.
- Jedes andere ASCII-Zeichen (0x00–0x7F) wird als `,XX` (uppercase HEX) kodiert.
- Nicht-ASCII (>=0x80) wird unverändert (UTF-8) durchgereicht.
- Decoder akzeptiert standardmäßig sowohl Groß- als auch Kleinbuchstaben in Hex (`[0-9A-Fa-f]`). Encoder erzeugt immer uppercase.
- Strict-Feature:
  - Nur `[0-9A-F]` erlaubt; Kleinbuchstaben -> Fehler.
  - Unmaskierte nicht-unreserved ASCII -> Fehler.

## Abweichung vs. (zukünftige) Go-Referenz
Noch keine bekannte Abweichung. Falls Unterschiede entdeckt werden, hier dokumentieren:

| Bereich | Beschreibung | Status |
|---------|--------------|--------|
|         |              |        |

## Fehler-Design
`ErrorKind` minimalistisch, Position (Byte-Index) optional. Kein `unsafe`, keine Panics, kein Logging.

## Performance-Ansatz
- ASCII Hot-Loop mit einfacher Branch-Condition.
- Vorab-Längenberechnung für `encode` zur Kapazitätsreserve.
- Zero-Alloc Pfad via `encode_into`.

## no_std Strategie
- `#![no_std]` wenn Feature `std` fehlt.
- `alloc` Feature stellt heap-basierte APIs bereit.
- Ohne `alloc`: nur `encode_into`, `encoded_len`, `validate_dnp`.

## Erweiterungen (Future Work)
- Streaming Decoder Iterator
- Fuzzing Targets (geplant unter `fuzz/`)
- Umfangreichere Golden-Test-Sammlung direkt aus PDF extrahiert


