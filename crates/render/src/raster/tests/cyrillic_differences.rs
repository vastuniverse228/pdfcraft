//! A non-embedded Arial or Times that names Cyrillic with `/Differences` (`afii10017` for А,
//! the cp1251 layout Word and older tools write) draws those letters. The standard stand-in
//! has no Cyrillic glyphs, so the letters were blank while the Latin on the same line drew
//! (#2908). They come from the bundled Inter face.

use super::render_first_page;

/// One page of `text` (a PDF literal). Code 192 is `afii10017` (А).
fn pdf(base_font: &str, text: &str) -> Vec<u8> {
    let content = format!("BT /F1 40 Tf 8 16 Td ({text}) Tj ET");
    format!(
        "%PDF-1.7
1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj
2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj
3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 160 70] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >> endobj
4 0 obj << /Length {} >> stream
{content}
endstream endobj
5 0 obj << /Type /Font /Subtype /TrueType /BaseFont /{base_font} /Encoding 6 0 R /FontDescriptor 7 0 R >> endobj
6 0 obj << /Type /Encoding /BaseEncoding /WinAnsiEncoding /Differences [192 /afii10017] >> endobj
7 0 obj << /Type /FontDescriptor /FontName /{base_font} /Flags 32 /FontBBox [0 -200 1000 900] /ItalicAngle 0 /Ascent 900 /Descent -200 /CapHeight 700 /StemV 80 >> endobj
trailer << /Root 1 0 R >>
%%EOF",
        content.len()
    )
    .into_bytes()
}

fn dark(bytes: &[u8]) -> usize {
    bytes.as_chunks::<4>().0.iter().filter(|p| p[0] < 128).count()
}

#[test]
fn non_embedded_arial_and_times_draw_a_cyrillic_difference() {
    for base in ["ArialMT", "Arial-BoldMT", "TimesNewRomanPSMT"] {
        let latin = dark(&render_first_page(pdf(base, "A")).rgba);
        let cyrillic = dark(&render_first_page(pdf(base, "\\300")).rgba);
        assert!(latin > 40, "{base}: the Latin A is drawn ({latin} dark pixels)");
        // Octal 300 is byte 192, the Differences entry afii10017.
        assert!(cyrillic > 40, "{base}: Cyrillic А (afii10017) is drawn ({cyrillic} dark pixels)");
    }
}
