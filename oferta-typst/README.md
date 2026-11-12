# Ofertă de preț – Vadini S.R.L. (Typst)

```
oferta-typst/
├── oferta.typ      template-ul (layout + calcule)
├── oferta.json     date de exemplu
├── oferta.pdf      exemplu generat
├── assets/         logo.png, stampila.png
└── fonts/          Liberation Sans (metric identic cu Arial, licență OFL)
```

## Compilare (CLI)

```sh
typst compile --font-path fonts oferta.typ                                   # folosește oferta.json
typst compile --font-path fonts --input file=client.json oferta.typ client.pdf
typst watch   --font-path fonts oferta.typ                                   # previzualizare live
```

`client.json` trebuie să fie în folderul proiectului (sau un subfolder), pentru că Typst nu citește fișiere din afara root-ului.

## Din Rust

Template-ul caută datele în `sys.inputs.data`, care poate fi:
- un **dicționar Typst** (`Dict`) construit din Rust, de exemplu cu `typst-as-lib` → `.with_inputs(...)`, sau
- un **șir JSON** (`serde_json::to_string(&oferta)`).

Dacă `data` lipsește, se folosește `sys.inputs.file` (cale JSON), apoi `oferta.json`.
Fonturile din `fonts/` și imaginile din `assets/` trebuie puse la dispoziția compilatorului (root = folderul proiectului).

## Structura datelor

| Câmp | Tip | Obligatoriu | Observații |
|---|---|---|---|
| `data_oferta` | text | da | ex. `"12.11.2026"` |
| `valabilitate` | text | nu | implicit `"30 de zile"` |
| `obiect_titlu` | text | da | rândul de sub „OFERTĂ DE PREȚ” |
| `obiect_descriere` | text | da | caseta „Obiectul ofertei” |
| `referinta` | text | nu | se ascunde dacă lipsește |
| `client.nume` | text | da | |
| `client.cui` / `client.adresa` / `client.contact` | text | nu | rândurile lipsă se ascund |
| `items[]` | listă | da | un rând de tabel per element |
| `items[].denumire` | text | da | |
| `items[].descriere` | text | nu | rând gri sub denumire |
| `items[].um` | text | da | ex. `"buc."`, `"ore"` |
| `items[].cant` | număr | da | 7.5 → „7,5” |
| `items[].pret` | număr | da | preț unitar fără TVA, în lei |
| `cota_tva` | număr | nu | implicit 21 |
| `termen_zile` | număr/text | nu | ex. `15` sau `"10-15"` |
| `conditii_plata` | text | nu | |
| `livrare` | text | nu | implicit `"Contracost"` |

Nr. curent, valoarea pe rând, totalul fără TVA, TVA-ul și totalul de plată se calculează în template
(rotunjire la 2 zecimale, format românesc `12.000,00`).
