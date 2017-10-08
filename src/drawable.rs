use ecs;
use sfml;
use graphics;

#[derive(Clone)]
pub struct Drawable(pub Box<::std::rc::Rc<Fn(&ecs::System, ecs::Entity, &mut graphics::Graphics)>>);

impl Drawable {
    pub fn new<F: Fn(&ecs::System, ecs::Entity, &mut graphics::Graphics) + 'static>(f: F) -> Drawable {
        Drawable(Box::new(::std::rc::Rc::new(f)))
    }
}

impl ::std::fmt::Debug for Drawable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Drawable")
    }
}

pub fn draw(sys: &ecs::System, gr: &mut graphics::Graphics) {
    use sfml::graphics::RenderTarget;

    gr.window.clear(&sfml::graphics::Color::BLUE);
    sys.run::<Drawable, _>(|sys: &ecs::System, ent: ecs::Entity| {
        let drawable = sys.borrow::<Drawable>(ent).unwrap();
        drawable.0(sys, ent, gr);
    }).unwrap();
    gr.window.display();
}
