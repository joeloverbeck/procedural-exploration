use bevy::prelude::*;
use std::{io, io::BufRead};

struct Input(String);

/// This example demonstrates you can create a custom runner (to update an app manually). It reads
/// lines from stdin and prints them from within the ecs.
fn my_runner(mut app: App) {
    println!("The fuck you mean, boy?");
    for line in io::stdin().lock().lines() {
        {
            let mut input = app.world.get_resource_mut::<Input>().unwrap();
            input.0 = line.unwrap();
        }
        app.update();
    }
}

fn print_system(input: Res<Input>) {
    println!("What the hell do you mean by '{}'?!", input.0);
}

fn main() {
    App::new()
        .insert_resource(Input(String::new()))
        .set_runner(my_runner)
        .add_system(print_system)
        .run();
}
