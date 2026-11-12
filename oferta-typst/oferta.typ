// =====================================================================
//  Ofertă de preț – Vadini S.R.L.
//
//  Datele vin din (în ordinea priorității):
//    1. sys.inputs.data  – dicționar trimis din Rust (typst-as-lib / typst-bake)
//                          sau șir JSON (CLI: --input data='{...}')
//    2. sys.inputs.file  – cale către un fișier JSON (CLI: --input file=x.json)
//    3. oferta.json      – implicit, pentru editare / previzualizare
//
//  Compilare:  typst compile --font-path fonts oferta.typ
//              typst compile --font-path fonts --input file=client.json oferta.typ client.pdf
// =====================================================================

#let d = {
  let inp = sys.inputs
  if "data" in inp {
    let v = inp.data
    if type(v) == str { json(bytes(v)) } else { v }
  } else {
    json(inp.at("file", default: "oferta.json"))
  }
}

// ---------- culori ----------
#let navy   = rgb("#1F3A5F")
#let accent = rgb("#C96A1B")
#let gri    = rgb("#5B6573")
#let fundal = rgb("#F2F4F7")
#let linie  = rgb("#D5DAE1")
#let sep    = [#h(3pt)·#h(3pt)]

// ---------- utilitare ----------
// 12000 -> "12.000,00"
#let lei(x) = {
  let cents = int(calc.round(x * 100))
  let neg = cents < 0
  let c = calc.abs(cents)
  let intreg = str(calc.quo(c, 100)).clusters()
  let zec = calc.rem(c, 100)
  let out = ""
  for (i, ch) in intreg.enumerate() {
    if i > 0 and calc.rem(intreg.len() - i, 3) == 0 { out += "." }
    out += ch
  }
  (if neg { "-" } else { "" }) + out + "," + (if zec < 10 { "0" } else { "" }) + str(zec)
}

// 1 -> "1", 2.5 -> "2,5", "10-15" -> "10-15"
#let nr(x) = {
  if type(x) == str { x }
  else if type(x) == int { str(x) }
  else if x == calc.floor(x) { str(int(x)) }
  else { str(x).replace(".", ",") }
}

#let are(x) = x != none and x != ""
#let eticheta(t) = text(7.5pt, weight: "bold", fill: gri, tracking: 1pt, upper(t))
#let randuri(..perechi) = {
  perechi.pos()
    .filter(p => are(p.at(1)))
    .map(p => [#text(fill: gri, p.at(0)) #p.at(1)])
    .join(linebreak())
}

// ---------- calcule ----------
#let items = d.items
#let valoare(it) = calc.round(it.cant * it.pret, digits: 2)
#let total = items.map(valoare).sum(default: 0)
#let cota = d.at("cota_tva", default: 21)
#let tva = calc.round(total * cota / 100, digits: 2)
#let total-cu-tva = total + tva

// ---------- antet / subsol ----------
#let antet = {
  grid(
    columns: (1.75cm, 1fr, auto),
    column-gutter: 0.3cm,
    align: (left + horizon, left + horizon, right + horizon),
    image("assets/logo.png", height: 2.05cm),
    [
      #text(15pt, weight: "bold", fill: navy, tracking: 0.4pt)[VADINI S.R.L.] \
      #text(8pt, fill: gri)[Sudură #sep Montaj #sep Confecții metalice]
    ],
    [
      #set text(8pt)
      #set par(leading: 0.55em)
      #text(fill: gri)[Cod fiscal:] *RO17785714* \
      Str. Principală nr. 465, Ardusat \
      jud. Maramureș, România \
      #text(fill: gri)[Tel.:] 0773 330 210 \
      #text(fill: gri)[E-mail:] hangobogdan\@sudurasimontaj.com
    ],
  )
  v(0.22cm, weak: false)
  line(length: 100%, stroke: 1.4pt + navy)
}

#let subsol = {
  set text(7pt, fill: gri)
  line(length: 100%, stroke: 0.5pt + linie)
  v(-0.12cm)
  grid(
    columns: (1fr, auto),
    [VADINI S.R.L. #sep CUI RO17785714 #sep Str. Principală nr. 465, Ardusat, jud. Maramureș],
    context [Pagina #counter(page).display() din #counter(page).final().first()],
  )
}

// ---------- pagină ----------
#set document(title: "Ofertă de preț – Vadini S.R.L.", author: "ing. Hango Bogdan-Ioan")
#set text(font: ("Liberation Sans", "Arial"), size: 9.5pt, lang: "ro", fill: rgb("#222222"))
#set par(leading: 0.6em, spacing: 0.9em)
#set page(
  paper: "a4",
  margin: (top: 4.25cm, bottom: 2cm, x: 2cm),
  header-ascent: 0.55cm,
  header: antet,
  footer-descent: 0.45cm,
  footer: subsol,
)

// ---------- titlu ----------
#grid(
  columns: (1fr, 6.7cm),
  column-gutter: 0.4cm,
  align: (left + top, left + top),
  stack(
    spacing: 0.32cm,
    text(22pt, weight: "bold", fill: navy, tracking: 1.5pt)[OFERTĂ DE PREȚ],
    text(10.5pt, fill: gri, d.obiect_titlu),
  ),
  block(
    width: 100%, fill: fundal,
    stroke: (left: 2.6pt + accent),
    inset: (left: 0.38cm, rest: 0.26cm),
  )[
    #set text(8.5pt)
    #grid(
      columns: (2.2cm, 1fr), row-gutter: 0.22cm,
      text(fill: gri)[Data:], [*#d.data_oferta*],
      text(fill: gri)[Valabilitate:], [*#d.at("valabilitate", default: "30 de zile")*],
    )
  ],
)

#v(0.45cm)

// ---------- beneficiar / obiect ----------
#grid(
  columns: (1fr, 1fr),
  column-gutter: 0.18cm,
  fill: fundal,
  inset: (x: 0.32cm, y: 0.3cm),
  [
    #set par(leading: 0.7em)
    #eticheta[Beneficiar]
    #v(0.05cm)
    #text(10pt, weight: "bold", d.client.nume) \
    #set text(8.5pt)
    #randuri(
      ("CUI:", d.client.at("cui", default: none)),
      ("Adresă:", d.client.at("adresa", default: none)),
      ("În atenția:", d.client.at("contact", default: none)),
    )
  ],
  [
    #set par(leading: 0.7em)
    #eticheta[Obiectul ofertei]
    #v(0.05cm)
    #set text(8.5pt)
    #d.obiect_descriere
    #if are(d.at("referinta", default: none)) [
      #v(0.05cm)
      #text(fill: gri)[Referință:] #d.referinta
    ]
  ],
)

#v(0.45cm)

Ca urmare a solicitării dumneavoastră, vă transmitem mai jos oferta noastră de preț:

#v(0.1cm)

// ---------- produse ----------
#table(
  columns: (0.95cm, 1fr, 1.3cm, 1.3cm, 2.9cm, 3.1cm),
  stroke: (x, y) => if y > 0 { (bottom: 0.6pt + linie) },
  fill: (x, y) => if y == 0 { navy },
  inset: (x, y) => (x: 0.17cm, y: if y == 0 { 0.17cm } else { 0.26cm }),
  align: (x, y) => (center, left, center, center, right, right).at(x) + horizon,
  table.header(
    ..([Nr.], [Denumire produs / serviciu], [U.M.], [Cant.],
       [Preț unitar \ (RON, fără TVA)], [Valoare \ (RON, fără TVA)])
      .map(h => text(8pt, weight: "bold", fill: white, h)),
  ),
  ..items.enumerate().map(((i, it)) => (
    str(i + 1),
    [
      #text(weight: "bold", it.denumire)
      #if are(it.at("descriere", default: none)) [\ #text(8pt, fill: gri, it.descriere)]
    ],
    it.um,
    nr(it.cant),
    lei(it.pret),
    text(weight: "bold", lei(valoare(it))),
  )).flatten(),
)

#v(0.1cm)

// ---------- totaluri ----------
#align(right, table(
  columns: (4.0cm, 3.85cm),
  stroke: (x, y) => if y < 2 { (bottom: 0.6pt + linie) },
  fill: (x, y) => if y == 2 { navy },
  inset: (x: 0.25cm, y: 0.24cm),
  align: (left + horizon, right + horizon),
  text(fill: gri)[Total fără TVA], [*#lei(total) RON*],
  text(fill: gri)[TVA (#nr(cota)%)], [*#lei(tva) RON*],
  text(10pt, weight: "bold", fill: white)[TOTAL DE PLATĂ],
  text(11pt, weight: "bold", fill: white)[#lei(total-cu-tva) RON],
))

#v(0.05cm)
#text(7.5pt, style: "italic", fill: gri)[Prețurile sunt exprimate în lei (RON). TVA calculat la cota de #nr(cota)%.]

#v(0.35cm)

// ---------- condiții ----------
#text(10pt, weight: "bold", fill: navy, tracking: 0.5pt)[CONDIȚII COMERCIALE]
#v(-0.05cm)
#table(
  columns: (5.1cm, 1fr),
  stroke: (x, y) => (bottom: 0.6pt + linie),
  inset: (x, y) => (left: if x == 1 { 0.15cm } else { 0pt }, right: 0pt, y: 0.19cm),
  ..(
    ("Termen de execuție",
      if are(d.at("termen_zile", default: none)) [#nr(d.termen_zile) zile lucrătoare de la confirmarea comenzii]),
    ("Condiții de plată", d.at("conditii_plata", default: none)),
    ("Livrare", d.at("livrare", default: "Contracost")),
  ).filter(r => are(r.at(1)))
   .map(r => (text(weight: "bold", fill: navy, r.at(0)), r.at(1)))
   .flatten(),
)

#v(0.35cm)

// ---------- încheiere + semnături (rămân împreună pe aceeași pagină) ----------
#block(breakable: false)[
Vă stăm la dispoziție pentru orice detalii suplimentare și așteptăm confirmarea dumneavoastră.

#v(0.08cm)
Cu stimă,

#v(0.25cm)

#grid(
  columns: (1fr, 1fr),
  column-gutter: 0.9cm,
  stack(
    spacing: 0.24cm,
    eticheta[Ofertant #sep Vadini S.R.L.],
    text(11pt, weight: "bold", fill: navy)[ing. Hango Bogdan-Ioan],
    image("assets/stampila.png", width: 4.1cm),
  ),
  stack(
    spacing: 0pt,
    eticheta[Beneficiar – de acord cu oferta],
    ..("Nume și prenume:", "Semnătura și ștampila:", "Data:").map(l => block(
      width: 100%,
      inset: (top: 0.55cm, bottom: 0.12cm),
      stroke: (bottom: 0.5pt + rgb("#9AA3AF")),
      text(8.5pt, fill: gri, l),
    )),
  ),
)
]
