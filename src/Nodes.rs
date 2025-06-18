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
    pub fn torque(&self, last_node: &Option<Node>, next_node: &Option<Node>) -> f32{
        let liqued_speed = 10.0;
        let liqued_density = 1.0;
        let force1;
        let force2;
        if let Some(last_node) = last_node {
            force1 = self.liqued_force(-(last_node.angle - self.angle), liqued_speed, liqued_density, &last_node); // the direction of where the pipe is looking
        } else {
            force1 = Vec2::ZERO;
        }
        if let Some(next_node) = next_node {
            force2 = next_node.liqued_force(-(self.angle - next_node.angle), liqued_speed, liqued_density, &self); // the direction of where the pipe is looking
        } else {
            force2 = Vec2::ZERO;
        }
        return Self::corss_product(force1, (self.pos - self.mid_pos)) + Self::corss_product(force2, (self.end_pos - self.mid_pos));

        // let force2: Vec2 = self.liqued_force(liqued_speed, liqued_density); // the direction of where the pipe is looking

    }

    pub fn liqued_force(&self, angle: f32, liqued_speed: f32, liqued_density: f32, other_node: &Node) -> Vec2{
        // if both angles are the same return zero
        if ((vec2(self.angle.cos(), self.angle.sin()) - vec2(other_node.angle.cos(), other_node.angle.sin())).length_squared() == 0.0) {return Vec2::ZERO}
        return liqued_density*liqued_speed*liqued_speed * (0.5*angle).sin() * 0.01 * (vec2(other_node.angle.cos(), other_node.angle.sin()) - vec2(self.angle.cos(), self.angle.sin())).normalize() /* 0.01 meanis that the pipe segment is one centimeter */
    }

    pub fn dumpin(&self) -> Vec2{
        let angle = self.angle + 0.5*PI;
        Vec2::ZERO
    }

    pub fn calculate_position(&mut self, last_node: &Option<Node>){
        if let Some(last_node) = last_node {
            self.pos = last_node.end_pos;
        } else {
            self.pos = vec2(0.0, -200.0);

        }
        self.end_pos = vec2(self.angle.cos(), self.angle.sin()) * self.radius*2.0 + self.pos;
        self.mid_pos = vec2(self.angle.cos(), self.angle.sin()) * self.radius + self.pos;
    }

    pub fn step(&mut self, dt: f32, last_node: &Option<Node>, next_node: &Option<Node>){
        let torque = self.torque(&last_node, &next_node);
        let inertia = self.mass*(1.0/12.0)*self.radius*self.radius;
        self.anglular_acceleration = torque/inertia;

        self.solver(dt);
        
        self.calculate_position(&last_node);
    }

    pub fn draw(&self, draw: &Draw, last_node: &Option<Node>, next_node: &Option<Node>, color: nannou::prelude::Hsl){
        if let Some(last_node) = last_node {
            let force1: Vec2 = self.liqued_force(last_node.angle - self.angle, 10.0, 0.1, &last_node); // the direction of where the pipe is looking
            // println!("{}, {}", last_node.end_pos, force1);
            draw.arrow().start(last_node.end_pos).end(last_node.end_pos + (force1)*1000.0).stroke_weight(2.0).color(RED);
            // let force2: Vec2 = -self.liqued_force(10.0, 1.0); // the direction of where the pipe is looking
            // draw.arrow().start(self.pos).end(self.pos + (-self.dumpin())*1000.0).stroke_weight(2.0).color(BLUE);
            // draw.arrow().start(self.pos).end(self.pos + (-self.dumpin() + force1)*1000.0).stroke_weight(2.0).color(PINK);
            // draw.arrow().start(self.pos).end(self.pos + vec2((connected_node.angle - self.angle).cos(), (connected_node.angle - self.angle).sin())*100.0).stroke_weight(2.0).color(PURPLE);
            // println!("angle: {}", (connected_node.angle - self.angle).rad_to_deg());
        }
        if let Some(next_node) = next_node {
            let force2: Vec2 = self.liqued_force(self.angle - next_node.angle, 10.0, 0.1, &next_node); // the direction of where the pipe is looking
            draw.arrow().start(next_node.pos).end(next_node.pos + (force2)*1000.0).stroke_weight(2.0).color(RED);
        }
        // draw.ellipse().radius(10.0).xy(self.pos).color(BLUE);
        // draw.ellipse().radius(10.0).xy(self.end_pos).color(BLUE);
        // draw.line().start(self.pos).end(self.end_pos).stroke_weight(10.0);
    }



    fn corss_product(vec1: Vec2, vec2: Vec2) -> f32{
        vec1.x*vec2.y - vec1.y*vec2.x
    }

    fn dot_product(vec1: Vec2, vec2: Vec2) -> f32{
        vec1.x*vec2.x + vec1.y*vec2.y
    }

    
}
