use winit::application::ApplicationHandler;
use winit::event_loop::{EventLoop, EventLoopProxy};
use winit::platform::web::EventLoopExtWebSys;

pub fn run<Message: 'static, App: ApplicationHandler<Message> + 'static>(
    make: impl FnOnce(EventLoopProxy<Message>) -> App,
) -> anyhow::Result<()> {
    let event_loop = EventLoop::<Message>::with_user_event().build()?;
    let app = make(event_loop.create_proxy());
    event_loop.spawn_app(app);
    Ok(())
}
