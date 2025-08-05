use gameput::run;


/// Gonna be dumping a lot here.
/// Gotta rewrite the app to be simpler. IDFK how to make the config work the way I want it to so for now I'll just have it read in a layer(s) config from a specified path, and work from there.
/// Also, gotta figure out the app design. it should be as simple as having one tokio task sit and listen for gamepad events and shovel them to the main app who will process them.
/// It may be a good idea to implement some kind of `Action` system, so that way e.g. people can change what buttons switch layers or quit. 
/// Therefore, the config will probably just be an `Option<Vec<(GamepadInput, Action)>>` or smth similar (`GamepadInput` will have to be made as well).
/// Also, it will be a good idea to outline the specs.
/// Full rewrite incoming.
/// Be prepaaaaaareeedddd~~!!!
/// 
/// TODO: the above

fn main() {
    run().expect("you FAILED (booo!)");
}
