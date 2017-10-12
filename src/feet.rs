use ecs;
use sfml;
use graphics;
use na;

use drawable;
use physics;

#[derive(Clone, Debug)]
pub struct Feet<'a> {
    pub circle: sfml::graphics::CircleShape<'a>,
}

pub fn create(sys: &mut ecs::System) -> ecs::Entity {
    use sfml::graphics::Transformable;
    use sfml::graphics::Shape;

    let ent = sys.new_entity();
    let mut feet = Feet {
        circle: sfml::graphics::CircleShape::new(::FEET_RADIUS, 16),
    };

    feet.circle.set_origin((::FEET_RADIUS, ::FEET_RADIUS));
    feet.circle.set_fill_color(&sfml::graphics::Color::GREEN);

    sys.add(ent, feet).unwrap();
    sys.add(ent, physics::Position(na::Point2::new(0.0, 0.0))).unwrap();
    sys.add(ent, physics::Velocity(na::Vector2::new(0.0, 200.0))).unwrap();
    sys.add(ent, physics::Acceleration(na::Vector2::new(0.0, 0.0))).unwrap();
    sys.add(ent, drawable::Drawable::new(draw)).unwrap();
    sys.add(ent, physics::Intelligent::new(update)).unwrap();
    sys.add(ent, graphics::EventHandler::new(handle_event)).unwrap();

    ent
}

pub fn handle_event(sys: &mut ecs::System, ent: ecs::Entity, ev: &sfml::window::Event) {
    if let &sfml::window::Event::KeyReleased { code: sfml::window::Key::Space, .. } = ev {
        // Jump
        let pos = sys.borrow::<physics::Position>(ent).unwrap().0;
        let vel = sys.borrow::<physics::Velocity>(ent).unwrap().0;

        let to_center = na::normalize(&pos.coords) * -1.0;
        let new_vel = vel + to_center * ::JUMP_SPEED;
        let new_pos = pos + to_center * 1.0;

        sys.borrow_mut::<physics::Velocity>(ent).unwrap().0 = new_vel;
        sys.borrow_mut::<physics::Position>(ent).unwrap().0 = new_pos;
    }
}

const MAX_RADIUS: f32 = ::STATION_RADIUS - ::FEET_RADIUS;

pub fn update(sys: &mut ecs::System, ent: ecs::Entity) {
    use sfml::graphics::Transformable;

    let pos = sys.borrow::<physics::Position>(ent).unwrap().0;

    let acc = if pos.coords.norm() >= MAX_RADIUS {
        let angle = pos.coords.y.atan2(pos.coords.x) + ::std::f32::consts::PI / 2.0;
        let vel_mag = ::ANGULAR_VELOCITY * MAX_RADIUS;
        let vel = na::Vector2::new(vel_mag * angle.cos(), vel_mag * angle.sin());

        sys.borrow_mut::<physics::Velocity>(ent).unwrap().0 = vel;

        pos.coords * -(::ANGULAR_VELOCITY * ::ANGULAR_VELOCITY)
    } else {
        na::Vector2::new(0.0, 0.0)
    };
    sys.borrow_mut::<physics::Acceleration>(ent).unwrap().0 = acc;

    // println!("{} {}", pos.coords.x, pos.coords.y);


    sys.borrow_mut::<Feet>(ent).unwrap().circle.set_position((pos.x, pos.y));
}

pub fn draw(sys: &ecs::System, ent: ecs::Entity, gr: &mut graphics::Graphics) {
    use sfml::graphics::RenderTarget;

    let feet = sys.borrow::<Feet>(ent).unwrap();

    gr.window.draw(&feet.circle);
}
