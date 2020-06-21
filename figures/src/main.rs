mod assembled;
mod assembled_a;
mod assembled_b;
mod assembled_c;
mod assembled_d;
mod assembled_e;
mod assembled_f;
mod assembled_g;
mod assembled_h;
mod io_config;
mod jumper_connections;
mod layered_cppn;
mod layered_cppn_b;
mod layout;
mod local_connections;
mod module;
mod single_cppn;
mod single_cppn_layout;

fn main() {
    assembled::create().save("outputs/assembled.tex");
    assembled_a::create().save("outputs/assembled_a.tex");
    assembled_b::create().save("outputs/assembled_b.tex");
    assembled_c::create().save("outputs/assembled_c.tex");
    assembled_d::create().save("outputs/assembled_d.tex");
    assembled_e::create().save("outputs/assembled_e.tex");
    assembled_f::create().save("outputs/assembled_f.tex");
    assembled_g::create().save("outputs/assembled_g.tex");
    assembled_h::create().save("outputs/assembled_h.tex");
    io_config::create().save("outputs/io_config.tex");
    jumper_connections::create().save("outputs/jumper_connections.tex");
    layered_cppn::create().save("outputs/layered_cppn.tex");
    layered_cppn_b::create().save("outputs/layered_cppn_b.tex");
    layout::create().save("outputs/layout.tex");
    local_connections::create().save("outputs/local_connections.tex");
    module::create().save("outputs/module.tex");
    single_cppn::create().save("outputs/single_cppn.tex");
    single_cppn_layout::create().save("outputs/single_cppn_layout.tex");
}
