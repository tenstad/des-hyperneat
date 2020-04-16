mod assembled;
mod blueprint;
mod local_connections;
mod jumper_connections;

fn main() {
    blueprint::create().save("outputs/blueprint.tex");
    assembled::create().save("outputs/assembled.tex");
    local_connections::create().save("outputs/local_connections.tex");
    jumper_connections::create().save("outputs/jumper_connections.tex");
}
