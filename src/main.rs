use std::{cell::RefCell, rc::Rc};

use nannou::{math::ConvertAngle, prelude::*};
use nannou_egui::{self, egui, Egui};

mod Nodes;
use Nodes::Node;
struct Model {
    // window: Window,
    egui: Egui,
    chain_length: usize,
    link_length: f32,
    speed: u32,
    chain: Vec<Node>
}

fn main() {
    nannou::app(model).update(update).run();
    
}

fn generate_chain(length: usize, link_length: f32, mass: f32) -> Vec<Node>{
    let mut nodes = Vec::with_capacity(length);
    nodes.push(Node::new(PI/2.0, link_length, mass));
    for i in 1..length {
        nodes[i-1].step(0.00001, None);

        nodes.push(Node::new(random_f32().abs() * PI + PI/2.0, link_length, mass));
    }
    // println!("{}", nodes.len());
    return nodes;
}


fn model(app: &App) -> Model {
    let window_id = app.new_window().view(view).raw_event(raw_window_event).build().unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let chain_length = 3;
    let link_length = 40.0;
    Model {
        egui,
        chain_length,
        chain: generate_chain(chain_length, link_length, 1.0),
        speed: 1,
        link_length,
    }
}



fn update(app: &App, model: &mut Model, update: Update) {
    // println!("{}", (app.mouse.position().y).atan2(app.mouse.position().x).rad_to_deg());
    render_egui(&mut model.egui, &mut model.chain_length, &mut model.link_length, &mut model.speed, &mut model.chain);
    let dt = 0.001;
    // println!("{:?}", &model.chain[9]);
    for _ in 0..model.speed*10{
        let clone_chain = model.chain.clone();
        for node in 1..model.chain_length{
            let current_link = Some(clone_chain[node-1].clone());
            model.chain[node].step(dt, current_link);
        }

    }
}
fn render_egui(egui: &mut Egui, chain_length: &mut usize, link_length: &mut f32, speed: &mut u32, chain: &mut Vec<Node>){
    let egui = egui;
    // egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();

    egui::Window::new("Rum window").show(&ctx, |ui| {
        ui.label("res"); // template
        let chain_length_slider = ui.add(egui::Slider::new(chain_length, 2..=500));
        let link_length_slider = ui.add(egui::Slider::new(link_length, 1.0..=100.0));

        ui.label("speed"); // template
        let speed = ui.add(egui::Slider::new(speed, 1..=100));
        let reset = ui.add(egui::Button::new("Reset"));
        // let slider = ui.add(egui::Slider::new(chain_length, 10..=500));

        if chain_length_slider.changed() || link_length_slider.changed() || reset.clicked() {
            *chain = generate_chain(*chain_length, *link_length, 1.0);
        }
    });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent){
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);


    draw.line().start(model.chain[0].pos).end(model.chain[0].end_pos).color(BLACK).stroke_weight(3.0);
    for node in 1..model.chain_length{
        let color = Hsl::new((node as f32)/(model.chain_length as f32)*360.0,1.0,0.5);
        draw.line().start(model.chain[node].pos).end(model.chain[node].end_pos).color(color).stroke_weight(3.0);


        let current_link = Some(model.chain[node-1].clone());
        model.chain[node].draw(&draw, &current_link, color);
    }

    // draw.polyline().weight(2.0).points(model.chain.iter().map(|link| link.end_pos)).color(BLACK);

    
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
