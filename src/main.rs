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
}

fn main() {
    nannou::app(model).update(update).run();
    
}

fn generate_chain(length: usize, link_rad: f32, mass: f32) -> Vec<Node>{
    let mut nodes = Vec::with_capacity(length);
    nodes.push(Node::new(PI/2.0, link_rad, mass));
    nodes[0].step(0.00001, &None, &None);
    for i in 1..length {
        nodes.push(Node::new(random_f32().abs() * 0.01*PI + PI/2.0, link_rad, mass));


        let last_node = Some(nodes[i-1].clone());
        nodes[i].step(0.00001, &last_node, &None);

    }
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
        debug: true,
    }
}



fn update(app: &App, model: &mut Model, update: Update) {
    // println!("{}", (app.mouse.position().y).atan2(app.mouse.position().x).rad_to_deg());
    render_egui(&mut model.egui, &mut model.chain_length, &mut model.link_length, &mut model.speed, &mut model.debug, &mut model.chain);
    let dt = 0.0001;
    // println!("{:?}", &model.chain[9]);
    for _ in 0..model.speed*10{
        let clone_chain = model.chain.clone();
        
        for node in 1..model.chain_length - 1{
            let last_node = Some(clone_chain[node-1].clone());
            let next_node = Some(clone_chain[node+1].clone());
            model.chain[node].step(dt, &last_node, &next_node);
        }

        let last_node = Some(clone_chain[model.chain_length - 2].clone());
        model.chain.last_mut().unwrap().step(dt, &last_node, &None);

    }
}
fn render_egui(egui: &mut Egui, chain_length: &mut usize, link_length: &mut f32, speed: &mut u32, debug: &mut bool, chain: &mut Vec<Node>){
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
fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        .join(app.exe_name().unwrap())
        // Name each file after the number of the frame.
        .join(format!("{:03}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}