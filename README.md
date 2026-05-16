# bizcard_pricer

A terminal-based print shop pricing calculator for business cards, written in Rust as practice

Built as a deliberate learning project to consolidate Rust Book Chapters 1 to 5: variables, functions, control flow, ownership, structs, methods, and associated functions. Every design decision in this codebase is intentional and documented.

---

## What it does

Takes a business card order (quantity, sided-ness, urgency) and computes a full price breakdown: sheets consumed, printing cost by click, material cost, labour, optional rush surcharge, and final price.

Money is stored as **integer cents throughout**, never as `f64`. This is the standard practice in financial software. `$7.50` is stored as `750`. The `currency()` function handles display conversion.

---

## Run it

```bash
git clone git@github.com:ariffzainal/bizcard-calcpricer-rust.git
cd bizcard-calcpricer-rust
cargo run
```

Requires Rust. Install via [rustup](https://rustup.rs) if needed.

---

## Costing model (v1)

| Factor | Value | Notes |
|---|---|---|
| Cards per box | 100 | Standard box |
| Cards per sheet | 20 | 13x19 inch sheet |
| Click cost | $0.40 | Per printed side, per sheet |
| Sheet cost | $2.00 | Art Card 270gsm |
| Labour | $30.00 flat | Per job. Assumption, adjust to real figure. |
| Rush multiplier | 1.5x | Applied to normal subtotal. Rounds to nearest cent. |

All constants are in one place at the top of `src/main.rs` under `// Costing facts`. Change a number there and it propagates everywhere.

---

## Structure

Two structs, one for inputs and one for outputs.

**`BizCardOrder`** holds what the customer chose.

```
double_sided: bool
urgent:       bool
quantity_boxes: u32
```

**`PriceBreakdown`** holds what the calculator worked out.

```
sheets_used:            u32   (rounded UP via div_ceil)
printing_cost_cent:     u32
material_cost_cent:     u32
labor_cost_cent:        u32
normal_total_cent:      u32   (subtotal before rush)
urgency_surcharge_cent: u32   (zero on a normal job)
final_price_cent:       u32
```

`PriceBreakdown::new(&order)` is the associated function that computes everything. `result.print_quote(&order)` prints the formatted quote to the terminal.

---

## What is hardcoded in v1

- One material only: Art Card 270gsm
- No volume discount (tiers not yet implemented)
- No terminal input (orders are hard-coded in `main`)
- Labour is a flat fee, not scaled by job size

These are deliberate scope limits, not oversights. See the upgrade plan below.

---

## Upgrade plan

### Chapter 6 upgrade: material choice as an enum

The comment `// CHAPTER 6 UPGRADE` marks every affected line in the source.

Rust's `enum` is the right tool for a fixed set of named choices. Material is exactly that: a closed list of stock options, not a free-form string.

What changes:

1. Add a `Material` enum with variants for each stock the shop carries, for example `ArtCard270`, `MattCoated150`, `Uncoated80`.
2. Add a `material: Material` field to `BizCardOrder`.
3. Replace `SHEET_COST_CENT` with a `match` on `order.material` that returns the correct cost per sheet for each variant.
4. The `match` is exhaustive: if a new material variant is added to the enum and the `match` is not updated, the compiler refuses to build. That is the compiler enforcing completeness for free.

This is also the natural point to add lamination as a separate `enum` field, since it has a similarly fixed set of options (none, matt, gloss).

### Volume discount

The comment `// NEXT STEP: volume discount` marks the line in `new` where this belongs.

The honest reason it was deferred: percentage discounts on integers require a deliberate rounding decision. Integer division truncates silently. That lesson needed to land cleanly on its own.

When implementing, the pattern is:

```
1 box          full price
2 to 4 boxes   n% off normal_total_cent
5 to 9 boxes   n% off
10+ boxes      n% off
```

The discount calculation follows the multiply-first rule to avoid early truncation:

```rust
// Apply a percentage discount expressed as integer basis points.
// 10% off is discount_pct = 10.
// Multiply first, divide last. Choose a rounding rule on purpose.
let discount_cent = (normal_total_cent * discount_pct + 50) / 100;
let final_price_cent = normal_total_cent - discount_cent;
```

The rounding rule (`+ 50` before `/ 100` is round-to-nearest) is a business decision. Document whichever rule the shop chooses. Round-up (`div_ceil`) slightly favours the shop. Round-down (plain `/`) slightly favours the customer.

Note that urgency and discount interact. The current v1 order of operations is: compute normal total, then apply urgency multiplier. A question to settle before implementing discount: does the rush multiplier apply before or after the volume discount? That is a pricing policy decision, not a Rust question.

### Terminal input (stretch goal)

The comment `// FUTURE UPGRADE: replace with typed input` marks the spot in `main`.

Typed input requires reading from `stdin`, parsing strings into the right types, and handling invalid input gracefully with `Result` and `match`. These land naturally after Chapter 9 (Error Handling). Do not attempt this before Chapter 9.

---

## Rust concepts practised here

| Concept | Where |
|---|---|
| `const` | All costing facts at the top |
| `struct` with named fields | `BizCardOrder`, `PriceBreakdown` |
| `#[derive(Debug)]` | Both structs |
| Associated function (`Type::new`) | `PriceBreakdown::new` |
| Method (`&self`) | `PriceBreakdown::print_quote` |
| Borrowing with `&` | `new(&order)`, `print_quote(&self, &order)` |
| `if` as an expression | `clicks_per_sheet`, `final_price_cent`, surcharge line |
| `div_ceil` for ceiling division | Sheet rounding in Step 1 |
| Integer money (cents) | Every money field |
| `format!` with `{:02}` padding | `currency()` function |

---

## Version history

**v1** (current): single material, flat labour, urgency multiplier, three hard-coded example orders. No discount, no input.

---

## Built with

- [Rust](https://www.rust-lang.org/)
- [The Rust Book](https://doc.rust-lang.org/book/) Chapters 1 to 5