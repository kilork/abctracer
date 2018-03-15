extern crate abctracer;

use abctracer::{Color, Environment, Surface, Vector, GObject, Ray};
use abctracer::geometry::Sphere;
use abctracer::geometry::Plane;
use abctracer::light::PointLight;
use abctracer::{render_scene, render_scene_supersampling_grid, DummyRenderBackend};
use abctracer::render::backend::htmlcanvas::HtmlCanvasBackend;
use abctracer::colors::{BLUE, RED, YELLOW};
use abctracer::mediums::GLASS;

fn main() {
    // let mut backend = DummyRenderBackend::new();
    let mut backend = HtmlCanvasBackend::new("scene1.html");
    let surface1 = Surface {
        k_a: 0.2,
        k_d: 0.5,
        k_s: 0.6,
        k_r: 0.0,
        k_t: 0.0,
        p: 30,
        color: YELLOW,
        medium: GLASS,
        n: Vector::from((0.0, 0.0, 0.0)),
    };

    let surface2 = Surface {
        color: RED,
        ..surface1
    };

    let surface3 = Surface {
        color: BLUE,
        ..surface1
    };

    let surface4 = Surface {
        k_a: 0.1,
        k_s: 0.4,
        k_d: 0.5,
        k_r: 0.4,
        color: BLUE,
        ..surface1
    };

    let surface1 = Surface {
        k_r: 0.3,
        ..surface1
    };

    let sphere_1 = Sphere::new(surface1, Vector::from((0.0, 1.0, 5.0)), 1.5);
    let sphere_2 = Sphere::new(surface2, Vector::from((-3.0, 0.0, 6.0)), 3.0);
    let sphere_3 = Sphere::new(surface3, Vector::from((3.0, 0.0, 4.0)), 1.0);
    let plane_4 = Plane::new(surface4, Vector::from((0.0, 1.0, 0.0)), 1.0);

    let point_light_1 = PointLight::new(Color::from(1.0), Vector::from((10.0, 5.0, -10.0)), 17.0);

    let mut environment = Environment::new();

    environment.add_solid(&sphere_1);
    environment.add_solid(&sphere_2);
    environment.add_solid(&sphere_3);
    environment.add_solid(&plane_4);

    environment.add_light(&point_light_1);

    environment.set_camera(
        &Vector::from(0.0),
        &Vector::from((0.0, 0.0, 1.0)),
        &Vector::from((0.0, 1.0, 0.0)),
    );

    render_scene_supersampling_grid(&mut environment, 1.3333333, 1.0, 640, 480, 3, 3, &mut backend).unwrap();
    // render_scene(&mut environment, 1.3333333, 1.0, 640, 480, &mut backend).unwrap();
}
