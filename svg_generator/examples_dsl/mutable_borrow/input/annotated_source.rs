fn main() {
    let mut <tspan data-hash="1">x</tspan> = <tspan class="fn" data-hash="0" hash="5">String::from</tspan>("hello");
    <tspan class="fn" data-hash="0" hash="6">world</tspan>(&amp;mut <tspan data-hash="1">&amp;x</tspan>);
    <tspan class="fn" data-hash="0" hash="7">println!</tspan>("{}", <tspan data-hash="1">x</tspan>)
}

fn world(<tspan data-hash="2">s</tspan> : &amp;mut String) {
    <tspan data-hash="2">s</tspan>.<tspan class="fn" data-hash="0" hash="8">push_str</tspan>(", world")
}