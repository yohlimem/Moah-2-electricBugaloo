use std::{cell::RefCell, clone, f32::consts::PI, rc::Rc};

use nannou::{color::{BLUE, PINK, PURPLE, RED}, glam::{vec2, Vec2}, math::ConvertAngle, Draw};


#[derive(Clone, Debug)]
pub struct Node {
    pub angle: f32,
    pub anglular_velocity: f32,
    pub anglular_acceleration: f32,
    pub radius: f32,
    pub mass: f32,
    pub pos: Vec2,
    pub end_pos: Vec2,
    pub mid_pos: Vec2,
    // pub connected_node: Option<Rc<RefCell<Node>>>,
}

impl Node {
    
    pub fn new(angle: f32, radius: f32, mass: f32, /*connected_node: Option<Rc<RefCell<Node>>>*/) -> Self{
        Node {/*connected_node, */angle, anglular_velocity: 0.0, anglular_acceleration: 0.0, radius, mass, pos: Vec2::ZERO, mid_pos: Vec2::ZERO, end_pos: Vec2::ZERO}
    }

    pub fn solver(&mut self, dt: f32){
        self.anglular_velocity += self.anglular_acceleration * dt;
        self.angle += self.anglular_velocity * dt;
    }
    /// distance is the vector from out position to the forces position
    pub fn torque(&self, connected_node: &Option<Node>) -> f32{
        let liqued_speed = 10.0;
        let liqued_density = 1.0;
        if let Some(connected_node) = connected_node { // TODO: fix angle
            let force1: Vec2 = connected_node.liqued_force(connected_node.angle - self.angle, liqued_speed, liqued_density); // the direction of where the pipe is looking
            // let force2: Vec2 = -self.liqued_force(liqued_speed, liqued_density); // the direction of where the pipe is looking
            // let recistance = force1 ity;
            return Self::corss_product(force1, (self.end_pos - self.mid_pos))/* + Self::corss_product(force2, (self.end_pos - self.mid_pos))*/;
        }

        // let force2: Vec2 = self.liqued_force(liqued_speed, liqued_density); // the direction of where the pipe is looking

        return 0.0;
    }

    pub fn liqued_force(&self, angle: f32, liqued_speed: f32, liqued_density: f32) -> Vec2{
        liqued_density*liqued_speed*liqued_speed * (0.5*angle).sin() * 0.01 * 2.0 * vec2(self.angle.cos(), self.angle.sin()) /* 0.01 meanis that the pipe segment is one centimeter */
    }

    pub fn dumpin(&self) -> Vec2{
        let angle = self.angle + 0.5*PI;
        Vec2::ZERO
        // vec2(angle.cos(), angle.sin()) * self.anglular_velocity * 0.1
    }

    pub fn calculate_position(&mut self, connected_node: &Option<Node>){
        if let Some(connected_node) = connected_node {
            self.pos = connected_node.end_pos;
        } else {
            self.pos = vec2(0.0, -200.0);

        }
        self.end_pos = vec2(self.angle.cos(), self.angle.sin()) * self.radius*2.0 + self.pos;
        self.mid_pos = vec2(self.angle.cos(), self.angle.sin()) * self.radius + self.pos;
    }

    pub fn step(&mut self, dt: f32, connected_node: Option<Node> /* clone */){
        let torque = self.torque(&connected_node);
        let inertia = self.mass*(1.0/12.0)*self.radius*self.radius;
        self.anglular_acceleration = torque/inertia;

        self.solver(dt);
        
        self.calculate_position(&connected_node);
    }

    pub fn draw(&self, draw: &Draw, connected_node: &Option<Node>, color: nannou::prelude::Hsl){
        if let Some(connected_node) = connected_node {
            let force1: Vec2 = connected_node.liqued_force(connected_node.angle - self.angle, 10.0, 0.1); // the direction of where the pipe is looking
            // let force2: Vec2 = -self.liqued_force(10.0, 1.0); // the direction of where the pipe is looking
            draw.arrow().start(connected_node.end_pos).end(connected_node.end_pos + (force1)*1000.0).stroke_weight(2.0).color(color);
            // draw.arrow().start(self.pos).end(self.pos + (-self.dumpin())*1000.0).stroke_weight(2.0).color(BLUE);
            // draw.arrow().start(self.pos).end(self.pos + (-self.dumpin() + force1)*1000.0).stroke_weight(2.0).color(PINK);
            // draw.arrow().start(self.pos).end(self.pos + vec2((connected_node.angle - self.angle).cos(), (connected_node.angle - self.angle).sin())*100.0).stroke_weight(2.0).color(PURPLE);
            // println!("angle: {}", (connected_node.angle - self.angle).rad_to_deg());
        }
        // draw.line().start(self.pos).end(self.end_pos).stroke_weight(10.0);
    }



    fn corss_product(vec1: Vec2, vec2: Vec2) -> f32{
        vec1.x*vec2.y - vec1.y*vec2.x
    }

    fn dot_product(vec1: Vec2, vec2: Vec2) -> f32{
        vec1.x*vec2.x + vec1.y*vec2.y
    }

    
}
