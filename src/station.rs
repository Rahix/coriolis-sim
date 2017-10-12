use ecs;
use sfml;
use graphics;

use drawable;
use physics;

#[derive(Clone, Debug)]
pub struct Station<'a> {
    pub tex: sfml::graphics::Texture,
    pub circle: sfml::graphics::CircleShape<'a>,
    pub align_view: bool,
}

pub fn create(sys: &mut ecs::System) -> ecs::Entity {
    //use sfml::graphics::Shape;
    use sfml::graphics::Transformable;

    let ent = sys.new_entity();
    let mut station = Station {
        tex: sfml::graphics::Texture::from_file("res/StationCross.png").unwrap(),
        circle: sfml::graphics::CircleShape::new(::STATION_RADIUS, 64),
        align_view: false,
    };
    //station.circle.set_fill_color(&sfml::graphics::Color::RED);
    station.circle.set_origin((::STATION_RADIUS, ::STATION_RADIUS));
    sys.add(ent, station).unwrap();
    sys.add(ent, physics::Rotation(0.0)).unwrap();
    sys.add(ent, physics::AngularVelocity(::ANGULAR_VELOCITY)).unwrap();
    sys.add(ent, drawable::Drawable::new(draw)).unwrap();
    sys.add(ent, graphics::EventHandler::new(handle_event)).unwrap();

    ent
}

pub fn handle_event(sys: &mut ecs::System, ent: ecs::Entity, ev: &sfml::window::Event) {
    if let &sfml::window::Event::KeyReleased { code: sfml::window::Key::V, .. } = ev {
        let mut station = sys.borrow_mut::<Station>(ent).unwrap();
        station.align_view = !station.align_view;
    }
}

pub fn draw(sys: &ecs::System, ent: ecs::Entity, gr: &mut graphics::Graphics) {
    use sfml::graphics::RenderTarget;
    use sfml::graphics::Transformable;

    let station = sys.borrow::<Station>(ent).unwrap();

    let mut sprite = sfml::graphics::Sprite::new();
    sprite.set_texture(&station.tex, true);
    sprite.set_origin((256.0, 256.0));
    let rotation = sys.borrow::<physics::Rotation>(ent).unwrap().0 / ::std::f32::consts::PI * 180.0;
    sprite.set_rotation(rotation);

    gr.window.draw(&station.circle);
    gr.window.draw(&sprite);

    if station.align_view {
        let mut view = gr.window.view().to_owned();
        view.set_rotation(rotation);
        gr.window.set_view(&view);
    }
}
