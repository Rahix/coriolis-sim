use sfml;
use graphics;
use ecs;

//use drawable;
use physics;

use station;
use feet;
use head;

pub fn coriolis_sim() {
    let mut graphics = graphics::Graphics::new();
    let mut system = ecs::System::new();

    let station = station::create(&mut system);
    let feet = feet::create(&mut system);
    let head = head::create(&mut system, feet);

    let mut previous_time = ::std::time::Instant::now();

    'gameloop: loop {
        let now = ::std::time::Instant::now();
        let elapsed = now.duration_since(previous_time);
        previous_time = ::std::time::Instant::now();
        let elapsed_f32 = elapsed.as_secs() as f32
           + elapsed.subsec_nanos() as f32 * 1e-9;

        while let Some(ref ev) = graphics.window.poll_event() {
            if let &sfml::window::Event::Closed = ev {
                break 'gameloop;
            } else {
                graphics::handle_event(&mut system, ev);
            }
        }

        physics::physics_update(&mut system, elapsed_f32);

        {
            use sfml::graphics::RenderTarget;

            graphics.window.clear(&sfml::graphics::Color::BLUE);

            station::draw(&system, station, &mut graphics);
            feet::draw(&system, feet, &mut graphics);
            head::draw(&system, head, &mut graphics);

            graphics.window.display();

            // Can't do that, drawing order issues :/
            // drawable::draw(&system, &mut graphics);
        }
    }
}
