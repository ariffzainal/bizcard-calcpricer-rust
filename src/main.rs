// bizcard_pricer_Structpractice
// v1: business cards only. Material is hard-coded as Art Card 270gsm.


// ---- Costing facts. One place to change them all. ----
// `const`: fixed values, known at compile time, never change while running.
// Naming them remove magic numbers from the maths

const CARDS_PER_BOX: u32 = 100; // one box holds 100 business cards
const CARDS_PER_SHEET: u32 = 20; // one 13 x 19 inch sheet yields 20 cards
const CLICK_COST_CENT: u32 = 200; // one printed side on one sheet
const SHEET_COST_CENT: u32 = 200; // Art card 270gsm, one sheet
const LABOR_COST_CENT: u32 = 3000; // flat fee per job (assumption)
const URGENCY_NUMERATOR: u32 = 3;   // 1.5x rush price, as the fraction 3/2
const URGENCY_DENOMINATOR: u32 = 2;

// soon we have to upgrade SHEET_COST_CENT becomes per material once material is a choice. (After Chapter 6 book)


// This struct holds the customer's CHOICES. It is the input to our calculator.

#[derive(Debug)] // this lets us print the whole struct for inspection. Without it, prog. will not compile when
                // we try to print the `order`
struct BizCardOrder {

    // Chapter 6 Upgrade: material choice goes here, as an enum.

    double_sided: bool, // Two possibilities, single or double sided. `true` means it is double sided.
    urgent: bool, // The job is either urgent or not
    quantity_boxes: u32, // is `u32`, never negative number and 32-bit give us 0 to 4 billion. 

}


//This struct holds the values to work out from an order. This is the output.
// All money fields are in cents and carry the _cent suffix to make that explicit

#[derive(Debug)]
struct PriceBreakdown {

    sheets_used: u32, // number of whole sheets required or used already rounded UP. A count not money.
    printing_cost_cent: u32, // USD or any currency, the ink/click cost
    material_cost_cent: u32, // USD or any currency, the card/paper material stock sheets in cents
    labor_cost_cent: u32, // USD or any currency, labour cost per job
    normal_total_cent: u32, // the normal job fee
    urgency_surcharge_cent: u32, 
    final_price_cent: u32, // USD or any currency, what the customer pays
}

impl PriceBreakdown {

    // ASSOCIATED FUNCTION (no `self`). Called as PriceBreakdown::new(&order).
    // It BORROWS the order (the &) to read it. Does not take ownership.

    fn new(order: &BizCardOrder) -> PriceBreakdown {
        // Boxes -> cards -> sheets (rounded up using div_ceil)
        let total_cards = order.quantity_boxes * CARDS_PER_BOX;
        let sheets_used = total_cards.div_ceil(CARDS_PER_SHEET);

        // Step 2: clicks. 1 printed side = 1 click. Double Sided = 2 per sheet.
        let clicks_per_sheet: u32 = if order.double_sided { 2 } else { 1 };
        let total_clicks = sheets_used * clicks_per_sheet;

        // Step 3 is cost components
        let printing_cost_cent = total_clicks * CLICK_COST_CENT;
        let material_cost_cent = sheets_used * SHEET_COST_CENT;
        let labor_cost_cent = LABOR_COST_CENT;

        // Step 4 the normal job total, before urgency multiplier.
        
        let normal_total_cent = 
            printing_cost_cent + material_cost_cent + labor_cost_cent;

        // Step 5: urgency as a MULTIPLIER, expressed as the fraction 3/2
        // Multiple first, divide LAST, so the fraction is not truncated early
        // Rounding Rule: Round to nearest. `+ URGENCY_DENOMINATOR / 2` before
        // the divide does this. Change to div_ceil for round-up or remove the 
        // `+ ...` for round-down. This is a business decision made on purpose.

        let final_price_cent = if order.urgent {
            (normal_total_cent * URGENCY_NUMERATOR + URGENCY_DENOMINATOR / 2)
                / URGENCY_DENOMINATOR
        } else {
            normal_total_cent
        };

        // Surchage is the GAP the multiplier created. Zero on a normal job.
        // Safe u32 subtraction: final_price_cent >= normal_total_cent always.
        let urgency_surcharge_cent = final_price_cent- normal_total_cent;

        // Step 5: build and hand back the finished struct (field init shorthand).

        PriceBreakdown { 
            sheets_used,
            printing_cost_cent,
            material_cost_cent,
            labor_cost_cent,
            normal_total_cent,
            urgency_surcharge_cent,
            final_price_cent,
        }
    }

    // METHOD (takes &self). Called as result.print_quote(&order).
    // Prints the full quote to the terminal. Does not take ownership of either value.
    fn print_quote(&self, order: &BizCardOrder) {
        println!("---- Quote ----");
        println!(
            "  {} box(es) | {} | {}",
            order.quantity_boxes,
            if order.double_sided { "double sided" } else { "single sided" },
            if order.urgent { "URGENT" } else { "standard" }
        );
        println!("  Sheets used:    {}", self.sheets_used);
        println!("  Printing cost:  {}", currency(self.printing_cost_cent));
        println!("  Material cost:  {}", currency(self.material_cost_cent));
        println!("  Labor cost:     {}", currency(self.labor_cost_cent));
        println!("  Subtotal:       {}", currency(self.normal_total_cent));
        if self.urgency_surcharge_cent > 0 {
            println!("  Rush surcharge: {}", currency(self.urgency_surcharge_cent));
        }
        println!("  Final price:    {}", currency(self.final_price_cent));
    }
}


// This function converts a money amount in cents into a human-readable currency string.
// 1950 becomes "$19.50", 705 becomes "$7.05", 40 becomes "$0.40"
fn currency(cent: u32) -> String {
    let currency_part = cent / 100; // whole currency
    let cent_part = cent % 100; // leftover cent
    format!("${}.{:02}", currency_part, cent_part)
}


fn main() {
    // Three example orders, hard-coded for now.
    // FUTURE UPGRADE: replace with typed input from the terminal.
 
    let order_a = BizCardOrder {
        double_sided: true,
        urgent: false,
        quantity_boxes: 3,
    };
 
    let order_b = BizCardOrder {
        double_sided: true,
        urgent: true,
        quantity_boxes: 3,
    };
 
    let order_c = BizCardOrder {
        double_sided: false,
        urgent: false,
        quantity_boxes: 10,
    };
 
    println!();
    PriceBreakdown::new(&order_a).print_quote(&order_a);
    println!();
    PriceBreakdown::new(&order_b).print_quote(&order_b);
    println!();
    PriceBreakdown::new(&order_c).print_quote(&order_c);
    println!();
}


