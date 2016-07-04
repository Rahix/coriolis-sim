extern crate sfml;

use self::sfml::system::{Vector2f, Clock};
use self::sfml::window::{ContextSettings, VideoMode, event, window_style};
use self::sfml::graphics::{RenderWindow, RenderTarget, View, CircleShape, Color};

use entity::{Entity, World, CoriolisEvent};
use station::Station;
use person::Person;

use std::f32::consts;

enum GameState {
    Uninitialized,
    Idle,
    Simulating,
    Exit,
}

pub struct CoriolisSim {
    state: GameState,
    world: World,
    window: sfml::graphics::RenderWindow,
    clock: sfml::system::Clock,
    view: sfml::graphics::View,
}

impl CoriolisSim {
    pub fn new() -> CoriolisSim {
        let mut window = match RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                              "Coriolis Station Simulation",
                                              window_style::CLOSE,
                                              &ContextSettings::default()) {
            Some(window) => window,
            None => panic!("Cannot create window!!"),
        };
        CoriolisSim {
            state: GameState::Uninitialized,
            world: World::new(),
            clock: Clock::new(),
            window: window,
            view: View::new().unwrap(),
        }
    }

    fn game_loop(&mut self) {
        match self.state {
            GameState::Idle => {
                }
            GameState::Simulating => {
                },
            _ => {},
        }
        for event in self.window.events() {
            match event {
                event::Closed => self.state = GameState::Exit,
                event::KeyReleased{code: code, alt: alt, ctrl: ctrl, shift: shift, system: system} => self.world.send_event(CoriolisEvent::Jump(500.0)),
                /*event::KeyPressed(ev) => {
                    if let GameState::Idle = self.state {
                        self.state = GameState::Simulating;
                    }
                    }*/
                _             => {                          },
            }
        }
        self.window.clear(&Color::blue());
        self.world.update(self.clock.get_elapsed_time());
        self.world.draw(&mut self.window);
        self.window.display();
    }

    pub fn start(&mut self) {
        if let GameState::Uninitialized = self.state {
        }
        else {
            return;
        }
        self.window = match RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                              "Coriolis Station Simulation",
                                              window_style::CLOSE,
                                              &ContextSettings::default()) {
            Some(window) => window,
            None => panic!("Cannot create window!!"),
        };
        let mut station = Station::new(300.0, consts::FRAC_PI_2);
        let mut person = Person::new(0.0, 0.0, consts::FRAC_PI_2, 280.0);
        self.world.add(station);
        self.world.add(person);
        self.view.set_center2f(0.0, 0.0);
        self.view.set_size2f(800.0, 600.0);
        self.window.set_view(&self.view);
        self.clock.restart();
        self.state = GameState::Idle;
        loop {
            if let GameState::Exit = self.state {
                break;
            }
            self.game_loop();
        }
    }
}
