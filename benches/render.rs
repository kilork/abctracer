#![feature(test)]
extern crate test;

extern crate abctracer;

#[cfg(test)]
mod tests {
    use super::*;

    use test::Bencher;
    use abctracer::{render_scene, Environment};
    use abctracer::render::backend::NullRenderBackend;

    #[bench]
    fn render_scene_empty_bench(b: &mut test::Bencher) {
        b.iter(|| {
            let mut backend = NullRenderBackend::new();
            let environment = Environment::new();
            render_scene(&environment, 1.0, 1.0, 100, 100, &mut backend);
        });
    }
}
