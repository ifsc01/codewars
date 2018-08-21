// https://www.codewars.com/kata/dna-to-rna-conversion

fn dna_to_rna(dna: &str) -> String {
  dna.replace("T", "U")
}

fn dna_to_rna(dna: &str) -> String {
    dna.chars().map(|c| {
        match c {
            'T' => 'U',
            c => c,
        }
    }).collect()
}

#[test]
fn returns_expected() {
  assert_eq!(dna_to_rna("TTTT"), "UUUU");
  assert_eq!(dna_to_rna("GCAT"), "GCAU");
}