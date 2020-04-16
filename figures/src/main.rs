mod blueprint;
mod assembled;

fn main() {
    blueprint::create().save("outputs/blueprint.tex");
    assembled::create().save("outputs/assembled.tex");
}
