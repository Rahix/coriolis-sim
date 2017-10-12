use ecs;
use sfml;
use graphics;
use na;

use drawable;
use physics;

#[derive(Clone, Debug)]
pub struct Head<'a> {
    pub circle: sfml::graphics::CircleShape<'a>,
    pub feet: ecs::Entity,
}

pub fn create(sys: &mut ecs::System, feet: ecs::Entity) -> ecs::Entity {
    use sfml::graphics::Transformable;
    use sfml::graphics::Shape;

    let ent = sys.new_entity();
    let mut head = Head {
        circle: sfml::graphics::CircleShape::new(::HEAD_RADIUS, 16),
        feet,
    };

    head.circle.set_origin((::HEAD_RADIUS, ::HEAD_RADIUS));
    head.circle.set_fill_color(&sfml::graphics::Color::CYAN);

    sys.add(ent, head).unwrap();
    sys.add(ent, physics::Position(na::Point2::new(0.0, -(::HEAD_RADIUS + ::FEET_RADIUS)))).unwrap();
    sys.add(ent, physics::Velocity(na::Vector2::new(0.0, 200.0))).unwrap();
    sys.add(ent, physics::Acceleration(na::Vector2::new(0.0, 0.0))).unwrap();
    sys.add(ent, drawable::Drawable::new(draw)).unwrap();
    sys.add(ent, physics::Intelligent::new(update)).unwrap();

    ent
}

pub fn update(sys: &mut ecs::System, ent: ecs::Entity) {
    use sfml::graphics::Transformable;

    let feet = sys.borrow::<Head>(ent).unwrap().feet;

    let pos_head = sys.borrow::<physics::Position>(ent).unwrap().0;
    let pos_feet = sys.borrow::<physics::Position>(feet).unwrap().0;

    let vel_head = sys.borrow::<physics::Velocity>(ent).unwrap().0;
    let vel_feet = sys.borrow::<physics::Velocity>(feet).unwrap().0;

    // Accelerate towards feet
    let to_feet = pos_feet.coords - pos_head.coords;
    let acc_mag = (to_feet.norm() - (::HEAD_RADIUS + ::FEET_RADIUS)) * 16000.0;
    let acc = {
        let acc = na::normalize(&to_feet) * acc_mag;
        let damp = (vel_head - vel_feet) * -12.0;

        // Stand up if feet are on the ground
        let standup = if pos_feet.coords.norm() >= (::STATION_RADIUS - ::FEET_RADIUS) {
            - na::normalize(&pos_head.coords) * 8000.0
        } else {
            na::Vector2::new(0.0, 0.0)
        };

        acc + damp + standup
    };

    sys.borrow_mut::<physics::Acceleration>(ent).unwrap().0 = acc;

    sys.borrow_mut::<Head>(ent).unwrap().circle.set_position((pos_head.x, pos_head.y));
}

pub fn draw(sys: &ecs::System, ent: ecs::Entity, gr: &mut graphics::Graphics) {
    use sfml::graphics::RenderTarget;

    let head = sys.borrow::<Head>(ent).unwrap();

    gr.window.draw(&head.circle);
}
