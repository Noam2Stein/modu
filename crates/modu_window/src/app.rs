use std::{mem::replace, sync::Arc};

use modu_gpu::{
    CurrentSurfaceTexture, Device, DeviceDescriptor, Instance, Queue, RequestAdapterOptions,
    Surface, SurfaceConfiguration, TextureView, TextureViewDescriptor,
};
use pollster::block_on;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

pub trait App: Sized {
    fn window_attributes() -> WindowAttributes {
        Window::default_attributes()
    }

    fn new(ctx: &AppContext) -> Self;

    fn event(&mut self, ctx: &AppContext, event: &AppEvent) {
        if let AppEvent::WindowEvent {
            window_id: _,
            event: WindowEvent::CloseRequested,
        } = event
        {
            ctx.event_loop.exit();
        }
    }

    fn update(&mut self, ctx: &AppContext) {
        let _ = ctx;
    }

    fn draw(&mut self, ctx: &AppContext, output: &TextureView) {
        let _ = (ctx, output);
    }

    fn exit(self, ctx: &AppContext) {
        let _ = ctx;
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct AppContext<'a> {
    pub event_loop: &'a ActiveEventLoop,
    pub window: &'a Window,
    pub device: &'a Device,
    pub queue: &'a Queue,
    pub surface: &'a Surface<'static>,
    pub surface_config: &'a mut SurfaceConfiguration,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum AppEvent {
    WindowEvent {
        window_id: WindowId,
        event: WindowEvent,
    },
    DeviceEvent {
        device_id: DeviceId,
        event: DeviceEvent,
    },
    Resumed,
    Suspended,
    MemoryWarning,
}

pub fn run_app<T>()
where
    T: App,
{
    let event_loop = EventLoop::new().expect("failed to create event loop");
    let mut app_runner = AppRunner::<T>::Uninitialized;
    event_loop
        .run_app(&mut app_runner)
        .expect("failed to run app");
}

enum AppRunner<T>
where
    T: App,
{
    Uninitialized,
    Initialized {
        window: Arc<Window>,
        device: Device,
        queue: Queue,
        surface: Surface<'static>,
        surface_config: SurfaceConfiguration,
        app: T,
    },
}

impl<T> ApplicationHandler for AppRunner<T>
where
    T: App,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Self::Initialized {
            window,
            device,
            queue,
            surface,
            surface_config,
            app,
        } = self
        {
            app.event(
                &AppContext {
                    event_loop,
                    window,
                    device,
                    queue,
                    surface,
                    surface_config,
                },
                &AppEvent::Resumed,
            );
            return;
        }

        let window_attributes = T::window_attributes().with_visible(false);
        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .expect("failed to create window"),
        );

        let instance = Instance::default();
        let adapter = block_on(instance.request_adapter(&RequestAdapterOptions::default()))
            .expect("failed to get adapter");
        let (device, queue) = block_on(adapter.request_device(&DeviceDescriptor::default()))
            .expect("failed to get device");

        let surface = instance
            .create_surface(window.clone())
            .expect("failed to create surface");
        let window_inner_size = window.inner_size();
        let mut surface_config = surface
            .get_default_config(&adapter, window_inner_size.width, window_inner_size.height)
            .expect("surface is not supported by adapter");
        surface.configure(&device, &surface_config);

        let app = T::new(&AppContext {
            event_loop,
            window: &window,
            device: &device,
            queue: &queue,
            surface: &surface,
            surface_config: &mut surface_config,
        });

        window.set_visible(true);

        *self = Self::Initialized {
            window,
            device,
            queue,
            surface,
            surface_config,
            app,
        };
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Self::Initialized {
            window,
            device,
            queue,
            surface,
            surface_config,
            app,
        } = self
        else {
            return;
        };

        match event {
            WindowEvent::RedrawRequested => 'redraw: {
                let mut reconfigure_surface = false;
                let surface_texture = loop {
                    match surface.get_current_texture() {
                        CurrentSurfaceTexture::Success(surface_texture) => break surface_texture,
                        CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                            reconfigure_surface = true;
                            break surface_texture;
                        }
                        CurrentSurfaceTexture::Timeout => break 'redraw,
                        CurrentSurfaceTexture::Occluded => break 'redraw,
                        CurrentSurfaceTexture::Outdated => {
                            surface.configure(device, surface_config);
                            continue;
                        }
                        CurrentSurfaceTexture::Lost => panic!("surface lost"),
                        CurrentSurfaceTexture::Validation => panic!("validation error"),
                    }
                };
                let output = surface_texture
                    .texture
                    .create_view(&TextureViewDescriptor::default());

                app.draw(
                    &AppContext {
                        event_loop,
                        window,
                        device,
                        queue,
                        surface,
                        surface_config,
                    },
                    &output,
                );

                surface_texture.present();
                if reconfigure_surface {
                    surface.configure(device, surface_config);
                }
            }
            WindowEvent::Resized(new_size) => {
                surface_config.width = new_size.width;
                surface_config.height = new_size.height;
                surface.configure(device, surface_config);
            }
            _ => {}
        }

        app.event(
            &AppContext {
                event_loop,
                window,
                device,
                queue,
                surface,
                surface_config,
            },
            &AppEvent::WindowEvent { window_id, event },
        );
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: DeviceId,
        event: DeviceEvent,
    ) {
        let Self::Initialized {
            window,
            device,
            queue,
            surface,
            surface_config,
            app,
        } = self
        else {
            return;
        };

        app.event(
            &AppContext {
                event_loop,
                window,
                device,
                queue,
                surface,
                surface_config,
            },
            &AppEvent::DeviceEvent { device_id, event },
        );
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let Self::Initialized {
            window,
            device,
            queue,
            surface,
            surface_config,
            app,
        } = self
        else {
            return;
        };

        app.update(&AppContext {
            event_loop,
            window,
            device,
            queue,
            surface,
            surface_config,
        });
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        let Self::Initialized {
            window,
            device,
            queue,
            surface,
            surface_config,
            app,
        } = self
        else {
            return;
        };

        app.event(
            &AppContext {
                event_loop,
                window,
                device,
                queue,
                surface,
                surface_config,
            },
            &AppEvent::Suspended,
        );
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        let Self::Initialized {
            window,
            device,
            queue,
            surface,
            mut surface_config,
            app,
        } = replace(self, Self::Uninitialized)
        else {
            return;
        };

        app.exit(&AppContext {
            event_loop,
            window: &window,
            device: &device,
            queue: &queue,
            surface: &surface,
            surface_config: &mut surface_config,
        });
    }

    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        let Self::Initialized {
            window,
            device,
            queue,
            surface,
            surface_config,
            app,
        } = self
        else {
            return;
        };

        app.event(
            &AppContext {
                event_loop,
                window,
                device,
                queue,
                surface,
                surface_config,
            },
            &AppEvent::MemoryWarning,
        );
    }
}
