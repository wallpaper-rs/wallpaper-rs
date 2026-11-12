fn particle(p: Sprite) -> Sprite {
    var o = p;
    o.size = p.size * (1.0 + param(0u).x * sin(frame.time * 6.0 + p.age));
    return o;
}
