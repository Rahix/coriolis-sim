use na;
use ecs;

#[derive(Clone, Debug)]
pub struct Position(pub na::Point2<f32>);

#[derive(Clone, Debug)]
pub struct Velocity(pub na::Vector2<f32>);

#[derive(Clone, Debug)]
pub struct Acceleration(pub na::Vector2<f32>);

#[derive(Clone, Debug)]
pub struct Rotation(pub f32);

#[derive(Clone, Debug)]
pub struct AngularVelocity(pub f32);

#[derive(Clone)]
pub struct Intelligent(pub Box<::std::rc::Rc<Fn(&mut ecs::System, ecs::Entity)>>);

impl Intelligent {
    pub fn new<F: Fn(&mut ecs::System, ecs::Entity) + 'static>(f: F) -> Intelligent {
        Intelligent(Box::new(::std::rc::Rc::new(f)))
    }
}

impl ::std::fmt::Debug for Intelligent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Intelligent")
    }
}

pub fn physics_update(sys: &mut ecs::System, delta_t: f32) {
    // Integrate acceleration
    sys.run_mut::<Acceleration, _>(|sys: &mut ecs::System, ent: ecs::Entity| {
        let new_vel = {
            let vel = sys.borrow::<Velocity>(ent).expect("Entity with acceleration does not have a velocity").0;
            let acc = sys.borrow::<Acceleration>(ent).unwrap().0;

            vel + acc * delta_t
        };
        sys.borrow_mut::<Velocity>(ent).unwrap().0 = new_vel;
    }).unwrap();

    // Move all objects with a velocity
    sys.run_mut::<Velocity, _>(|sys: &mut ecs::System, ent: ecs::Entity| {
        let new_pos = {
            let pos = sys.borrow::<Position>(ent).expect("Entity with velocity does not have a position").0;
            let vel = sys.borrow::<Velocity>(ent).unwrap().0;

            na::Translation2::from_vector(vel * delta_t) * pos
        };
        sys.borrow_mut::<Position>(ent).unwrap().0 = new_pos;
    }).unwrap();

    // Rotate all entities with an angular velocity
    sys.run_mut::<AngularVelocity, _>(|sys: &mut ecs::System, ent: ecs::Entity| {
        let new_rot = {
            let rot = sys.borrow::<Rotation>(ent).expect("Entity with angular velocity does not have a rotation");
            let vel = sys.borrow::<AngularVelocity>(ent).unwrap();
            rot.0 + vel.0 * delta_t
        };
        sys.borrow_mut::<Rotation>(ent).unwrap().0 = new_rot;
    }).unwrap();

    // Update all intelligent entities
    sys.run_mut::<Intelligent, _>(|sys: &mut ecs::System, ent: ecs::Entity| {
        let intelligent = sys.get::<Intelligent>(ent).unwrap();
        intelligent.0(sys, ent);
    }).unwrap();
}
