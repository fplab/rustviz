/* --- BEGIN Variable Definitions ---
Struct r{w, h}; StaticRef rect;
Function area(); Function println!()
--- END Variable Definitions --- */
struct Rect {
    w: u32,
    h: u32,
}

fn main() {
    let r = Rect { // !{ Bind(None->r) }
        w: 30, // !{ Bind(None->w) }
        h: 50 // !{ Bind(None->h) }
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&r) // !{ PassByStaticReference(r->area()) }
    );
} // !{ StructBox(r->h), GoOutOfScope(w), GoOutOfScope(h), GoOutOfScope(r) }

fn area(rect: &Rect) -> u32 { // !{ InitializeParam(rect) }
    rect.w * rect.h
} // !{ GoOutOfScope(rect) }