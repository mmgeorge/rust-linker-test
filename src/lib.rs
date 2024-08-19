use polars::prelude::*;
use std::collections::HashMap;
use winit::event_loop;

#[derive(serde::Deserialize)]
struct JsonStruct {
    _name: String,
}

struct MyApp {}
impl winit::application::ApplicationHandler for MyApp {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        todo!()
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
        todo!()
    }
}

/// This is VERY contrived and will panic
pub fn use_dependencies() {
    // use tokio
    let rt = tokio::runtime::Builder::new_multi_thread().build();

    // use polars
    let file = std::fs::File::open("some_file.parquet").unwrap();
    let _df = ParquetReader::new(file).finish().unwrap();

    rt.unwrap().block_on(async {
        let mut map = HashMap::new();
        map.insert("lang", "rust");
        map.insert("body", "json");

        // use wgpu
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .expect("Unable to create context");

        let (_device, _queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    memory_hints: wgpu::MemoryHints::Performance,
                    label: None,
                    required_features: wgpu::Features::INDIRECT_FIRST_INSTANCE,
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Unable to create device");

        // use reqwest + serde
        let _result = reqwest::get("<does-not-exist>")
            .await
            .unwrap()
            .json::<JsonStruct>()
            .await
            .unwrap();

        // use winit
        let mut app = MyApp {};
        let event_loop = event_loop::EventLoop::new().unwrap();

        event_loop.run_app(&mut app).unwrap();
    })
}
