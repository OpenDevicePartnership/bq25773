# Vendor documentation

Layout-preserving text extracts let code reviews quote the BQ25773 vendor
specification with stable, greppable line anchors. The code-review skill at
`.github/skills/code-review/SKILL.md` uses both documents below.

## Copyright and authority

**These files reproduce Texas Instruments documentation. They remain TI's
copyright and are not covered by this repository's MIT license.** Vendor
notices are retained. The extracts support specification review; they do not
replace the official PDFs. Text extraction does not preserve all diagrams,
waveforms or formatting: consult the PDF when those carry the requirement.

Official sources:

- Product: <https://www.ti.com/product/BQ25773>
- Datasheet: <https://www.ti.com/lit/ds/symlink/bq25773.pdf>
- Application note: <https://www.ti.com/lit/pdf/sdaa093>
- TI terms: <https://www.ti.com/legal/terms-conditions/terms-of-use.html>

The extracts and this skill are repository-only review material, excluded
from the published crate by the `Cargo.toml` packaging `include` list.

## Provenance

The source PDFs were supplied locally in `~/Downloads/bq/` and extracted on
2026-10-09 with Poppler `pdftotext` **25.07.0**, using `-layout -enc UTF-8`.
Only CRLF/CR line endings are normalized to LF; page-break form feeds,
vendor wording, notices and text-extraction artifacts are retained.
PDFs are not committed.

The local `.gitattributes` keeps extracts LF-terminated on checkout, including
on Windows with `core.autocrlf` enabled.

| Extract | Source PDF | Document and revision | Source PDF SHA-256 |
|---|---|---|---|
| `datasheet.txt` | `bq25773.pdf` | BQ25773 40V, I2C, 2- to 5-Cell, Narrow VDC Quasi Dual Phase Buck-Boost Battery Charge Controller With System Power Monitor and Processor Hot Monitor — SLUSEK7, September 2024 | `610e4625db1ce04be71c07e06094f14dafadae76d186d02a585d172489ca1397` |
| `sdaa093.txt` | `sdaa093.pdf` | Minimizing Charger EMI with DRSS: Practical Benefits and Lab Validation Techniques — SDAA093, February 2026 | `9cc8c2ceba993c4395e945a23ef5fc0312ecc7743c379ae79520ff6360d242a7` |

Future application notes or errata may be added with equivalent provenance.
Every vendor `.txt` extract in this directory is in review scope.

## Regeneration

Run from the repository root with the original PDFs and the recorded Poppler
version. This PowerShell recipe verifies **both** hashes before extracting:

```powershell
$sources = @(
    @{ Pdf = "$HOME/Downloads/bq/bq25773.pdf"; Text = 'docs/vendor/datasheet.txt';
       Hash = '610e4625db1ce04be71c07e06094f14dafadae76d186d02a585d172489ca1397' },
    @{ Pdf = "$HOME/Downloads/bq/sdaa093.pdf"; Text = 'docs/vendor/sdaa093.txt';
       Hash = '9cc8c2ceba993c4395e945a23ef5fc0312ecc7743c379ae79520ff6360d242a7' }
)
foreach ($source in $sources) {
    if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source.Pdf).Hash -ne $source.Hash) {
        throw "Source PDF changed: $($source.Pdf). Stop and review all affected citations."
    }
}
$utf8 = [System.Text.UTF8Encoding]::new($false)
foreach ($source in $sources) {
    pdftotext -layout -enc UTF-8 $source.Pdf $source.Text
    if ($LASTEXITCODE -ne 0) { throw "Extraction failed: $($source.Pdf)" }
    $text = [System.IO.File]::ReadAllText($source.Text, $utf8)
    $text = $text.Replace("`r`n", "`n").Replace("`r", "`n")
    [System.IO.File]::WriteAllText($source.Text, $text, $utf8)
}
```

On other platforms, use the same `pdftotext -layout -enc UTF-8` options after
checking the SHA-256 hashes and ensure LF newlines. Do not remove form feeds,
reflow tables or clean up apparent vendor typos: that changes line anchors.
Different Poppler versions can also change extraction output.

TI URLs are unversioned and may serve different bytes later. A hash mismatch
requires a deliberate source update and re-check of every skill citation,
not a silent overwrite. Only regenerate a missing or demonstrably stale
extract. Cite a committed `docs/vendor/<file>.txt:<line-range>` and verbatim
quote; neither a remembered section number nor the skill's summary table
alone is evidence.
