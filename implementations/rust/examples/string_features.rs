use jyaml::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== JYAML String Features Examples ===\n");

    // Example 1: String escapes in double quotes
    println!("1. Double-quoted string escapes:");
    let escaped_jyaml = r#""Hello\nWorld\t\"Quote\"""#;
    let escaped = parse(escaped_jyaml)?;
    println!("  JYAML: {}", escaped_jyaml);
    println!("  Parsed: {:?}\n", escaped);

    // Example 2: Limited escapes in single quotes
    println!("2. Single-quoted string (limited escapes):");
    let single_jyaml = r#"'Can\'t stop this\nLiteral text'"#;
    let single = parse(single_jyaml)?;
    println!("  JYAML: {}", single_jyaml);
    println!("  Parsed: {:?}\n", single);

    // Example 3: Unicode escapes
    println!("3. Unicode escapes:");
    let unicode_jyaml = r#""\u00A9 2023 \u{1F980} JYAML""#;
    let unicode = parse(unicode_jyaml)?;
    println!("  JYAML: {}", unicode_jyaml);
    println!("  Parsed: {:?}\n", unicode);

    // Example 4: Literal block scalar (pipe)
    println!("4. Literal block scalar (|):");
    let literal_jyaml = r#"
|
  Line 1
  Line 2
    Indented line
  Line 3
"#;
    let literal = parse(literal_jyaml)?;
    println!("  JYAML:{}", literal_jyaml);
    println!("  Parsed: {:?}\n", literal);

    // Example 5: Literal block scalar with strip (|-)
    println!("5. Literal block scalar with strip (|-):");
    let literal_strip_jyaml = r#"
|-
  No trailing newline
  Second line
"#;
    let literal_strip = parse(literal_strip_jyaml)?;
    println!("  JYAML:{}", literal_strip_jyaml);
    println!("  Parsed: {:?}\n", literal_strip);

    // Example 6: Folded block scalar (>)
    println!("6. Folded block scalar (>):");
    let folded_jyaml = r#"
>
  This is a long line
  that will be folded
  into a single line.
  
  This is a new paragraph.
"#;
    let folded = parse(folded_jyaml)?;
    println!("  JYAML:{}", folded_jyaml);
    println!("  Parsed: {:?}\n", folded);

    // Example 7: Folded block scalar with strip (>-)
    println!("7. Folded block scalar with strip (>-):");
    let folded_strip_jyaml = r#"
>-
  This text will be folded
  and the final newline
  will be stripped.
"#;
    let folded_strip = parse(folded_strip_jyaml)?;
    println!("  JYAML:{}", folded_strip_jyaml);
    println!("  Parsed: {:?}\n", folded_strip);

    println!("=== String features examples completed! ===");
    Ok(())
}