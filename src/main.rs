use std::{cell::RefCell, fmt::{format, Write}, rc::Rc};

use nannou::{image::gif::GifEncoder, math::ConvertAngle, prelude::*};
use nannou_egui::{self, egui, Egui};

mod Nodes;
use Nodes::Node;
struct Model {
    // window: Window,
    egui: Egui,
    chain_length: usize,
    link_length: f32,
    speed: u32,
    chain: Vec<Node>,
    debug: bool,
    start_energy: f32,
}

fn main() {
    nannou::app(model).update(update).run();
    
}

fn generate_chain(length: usize, link_rad: f32, mass: f32) -> Vec<Node>{
    let mut nodes = Vec::with_capacity(length);
    nodes.push(Node::new(PI/2.0, link_rad, mass));
    nodes[0].anglular_acceleration = nodes[0].acceleration_function(&None, &None);
    for i in 1..length {
        nodes.push(Node::new((random_f32() - 0.5)*2.0 * 0.2*PI + PI/2.0, link_rad, mass));


        let last_node = Some(nodes[i-1].clone());
        nodes[1].anglular_acceleration = nodes[i].acceleration_function(&last_node, &None);

    }
    return nodes;
}


fn model(app: &App) -> Model {
    let window_id = app.new_window().view(view).raw_event(raw_window_event).build().unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let chain_length = 5;
    let link_length = 30.0;
    let start_chain = generate_chain(chain_length, link_length, 1.0);
    let mut model = Model {
        egui,
        chain_length,
        start_energy: 0.0,
        chain: start_chain,
        speed: 1,
        link_length,
        debug: true,
    };
    
    // for _ in 0..30{
    //     step_all_no_energy(&mut model.chain,  model.start_energy, 0.0001);
    // }
    // model.start_energy = calculate_total_elastic_energy(&model.chain) + calculate_total_angular_kinetic_energy(&model.chain) + calculate_total_water_kinetic_energy(&model.chain);
    // println!("{}", model.start_energy );
    return model;
}



fn update(app: &App, model: &mut Model, update: Update) {
    // println!("{}", (app.mouse.position().y).atan2(app.mouse.position().x).rad_to_deg());
    render_egui(&mut model.egui, &mut model.chain_length, &mut model.link_length, &mut model.speed, &mut model.debug, &mut model.chain, &mut model.start_energy);
    let dt = 0.001;
    // println!("{:?}", &model.chain[9]);
    for _ in 0..model.speed{
        model.chain = step_all(model.chain.clone(), model.start_energy, dt);
    }
    // println!("new_energy: {}, elastic_energy: {}, kinetic_energy: {}", calculate_total_elastic_energy(&model.chain) + calculate_total_water_kinetic_energy(&model.chain) + calculate_total_angular_kinetic_energy(&model.chain), calculate_total_elastic_energy(&model.chain), calculate_total_angular_kinetic_energy(&model.chain));
    // println!("{}", (model.start_energy / (calculate_total_elastic_energy(&model.chain) + calculate_total_water_kinetic_energy(&model.chain) + calculate_total_angular_kinetic_energy(&model.chain))));
}
fn render_egui(egui: &mut Egui, chain_length: &mut usize, link_length: &mut f32, speed: &mut u32, debug: &mut bool, chain: &mut Vec<Node>, start_energy: &mut f32){
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
        let debug_toggle = ui.add(egui::Checkbox::new(debug, "Debug"));
        // let slider = ui.add(egui::Slider::new(chain_length, 10..=500));

        if chain_length_slider.changed() || link_length_slider.changed() || reset.clicked() {
            *chain = generate_chain(*chain_length, *link_length, 1.0);
        //     for _ in 0..30{
        //         step_all_no_energy(chain, *start_energy, 0.001);
        //     }
        //     *start_energy = calculate_total_elastic_energy(&chain) + calculate_total_angular_kinetic_energy(&chain) + calculate_total_water_kinetic_energy(&chain);
        }
    });
    
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent){
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    if model.debug {
        draw.line().start(model.chain[0].pos).end(model.chain[0].end_pos).color(BLACK).stroke_weight(3.0);
        
        draw.line().start(model.chain[model.chain_length - 1].pos).end(model.chain[model.chain_length - 1].end_pos).color(BLACK).stroke_weight(3.0);
        
        let last_node = Some(model.chain[model.chain_length - 2].clone());
        model.chain.last().unwrap().draw(&draw, &last_node, &None, Hsl::new(0.0,0.0,0.0));
        
        
        // let next_node = Some(model.chain[1].clone());
        // model.chain.first().unwrap().draw(&draw, &None, &next_node, Hsl::new(0.0,0.0,0.0));
        
        for node in 1..model.chain_length - 1{
            let color = Hsl::new((node as f32)/(model.chain_length as f32)*360.0,1.0,0.5);
            draw.line().start(model.chain[node].pos).end(model.chain[node].end_pos).color(color).stroke_weight(3.0);
    
    
            let last_node = Some(model.chain[node-1].clone());
            let next_node = Some(model.chain[node+1].clone());
            model.chain[node].draw(&draw, &last_node, &next_node, color);
        }

    } else {
        draw.polyline().weight(2.0).points(model.chain.iter().map(|link| link.end_pos)).color(BLACK);

    }


    
    draw.to_frame(app, &frame).unwrap();
    
    // app.main_window().capture_frame(captured_frame_path(&app, &frame)); // https://github.com/nannou-org/nannou/blob/master/examples/draw/draw_capture.rs
    
    model.egui.draw_to_frame(&frame).unwrap();
    
}

fn calculate_total_elastic_energy(chain: &Vec<Node>) -> f32{
    let mut sum = 0.0;
    for link in chain.windows(2) {
        sum += (link[0].angle - link[1].angle).sin()*(link[0].angle - link[1].angle).sin()*100.0/(2.0);
    }
    return sum;
}

fn calculate_total_angular_kinetic_energy(chain: &Vec<Node>) -> f32{
    let mut sum = 0.0;
    for link in chain {
        sum += link.anglular_velocity*link.anglular_velocity*(link.mass*link.radius*link.radius*(1.0/12.0))/2.0;    
    }
    return sum;
    
}
fn calculate_total_water_kinetic_energy(chain: &Vec<Node>) -> f32{
    return Node::LIQUID_SPEED*Node::LIQUID_SPEED*(Node::LIQUID_DENSITY*Node::LIQUID_SPEED*0.01*0.01)/2.0 * chain.len() as f32; // V^2 * (density*speed*dt*surface_area_of_the_pipe_hole)/2
    // let mut sum = 0.0;
    // for link in chain.windows(2) {
    //     sum += Node::LIQUID_DENSITY*Node::LIQUID_SPEED*Node::LIQUID_SPEED *0.01*(vec2(-link[1].angle.sin(), link[1].angle.cos()) - vec2(-link[0].angle.sin(), link[0].angle.cos())).normalize_or_zero();
    // }
    // return 
}

pub fn step_all(chain: Vec<Node>, start_energy: f32, dt: f32) -> Vec<Node>{
    let mut chain = chain;
    let clone_chain = chain.clone();
    let new_energy = calculate_total_elastic_energy(&clone_chain) + calculate_total_water_kinetic_energy(&clone_chain)+calculate_total_angular_kinetic_energy(&clone_chain);
    for node in 1..chain.len() - 1{
        let last_node = Some(clone_chain[node-1].clone());
        let next_node = Some(clone_chain[node+1].clone());
        chain[node].anglular_acceleration = chain[node].acceleration_function(&last_node, &next_node);
        chain[node].solver(dt, &last_node, &next_node);
    }
    
    let last_node = Some(clone_chain[chain.len() - 2].clone());
    chain.last_mut().unwrap().anglular_acceleration = chain.last_mut().unwrap().acceleration_function(&last_node, &None);
    chain.last_mut().unwrap().solver(dt, &last_node, &None);
    return chain;
}
