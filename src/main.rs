mod application;
use application::App;

fn main() {
    let mut app = App::new();
    app.run();

    println!("App closing.");
}
