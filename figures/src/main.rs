mod assembled;
mod assembled_a;
mod assembled_b;
mod assembled_c;
mod assembled_d;
mod assembled_e;
mod assembled_f;
mod assembled_g;
mod assembled_h;
mod blueprint;
mod local_connections;
mod jumper_connections;

fn main() {
    blueprint::create().save("outputs/blueprint.tex");
    assembled::create().save("outputs/assembled.tex");
    assembled_a::create().save("outputs/assembled_a.tex");
    assembled_b::create().save("outputs/assembled_b.tex");
    assembled_c::create().save("outputs/assembled_c.tex");
    assembled_d::create().save("outputs/assembled_d.tex");
    assembled_e::create().save("outputs/assembled_e.tex");
    assembled_f::create().save("outputs/assembled_f.tex");
    assembled_g::create().save("outputs/assembled_g.tex");
    assembled_h::create().save("outputs/assembled_h.tex");
    local_connections::create().save("outputs/local_connections.tex");
    jumper_connections::create().save("outputs/jumper_connections.tex");
}
