use sfml;
use ecs;

pub struct Graphics {
    pub window: sfml::graphics::RenderWindow,
}

impl Graphics {
    pub fn new() -> Graphics {
        use sfml::graphics::RenderTarget;

        let mut window = sfml::graphics::RenderWindow::new(sfml::window::VideoMode::new(800, 600, 32),
                                                       "Coriolis Station Simulation",
                                                       sfml::window::Style::CLOSE,
                                                       &sfml::window::ContextSettings::default());
        window.set_view(&sfml::graphics::View::new(sfml::system::Vector2::new(0.0, 0.0), sfml::system::Vector2::new(800.0, 600.0)));
        Graphics {
            window,
        }
    }
}

#[derive(Clone)]
pub struct EventHandler(pub Box<::std::rc::Rc<Fn(&mut ecs::System, ecs::Entity, &sfml::window::Event)>>);

impl EventHandler {
    pub fn new<F: Fn(&mut ecs::System, ecs::Entity, &sfml::window::Event) + 'static>(f: F) -> EventHandler {
        EventHandler(Box::new(::std::rc::Rc::new(f)))
    }
}

impl ::std::fmt::Debug for EventHandler {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "EventHandler")
    }
}

pub fn handle_event(sys: &mut ecs::System, ev: &sfml::window::Event) {
    sys.run_mut::<EventHandler, _>(|sys: &mut ecs::System, ent: ecs::Entity| {
        let handler = sys.get::<EventHandler>(ent).unwrap();
        handler.0(sys, ent, ev);
    }).unwrap();
}
