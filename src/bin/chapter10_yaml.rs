use rtc::io::yaml::parse_yaml;

fn main() {
    let (cam, world) = parse_yaml("samples/chapter10.yml").unwrap();

    let canvas = cam.unwrap().render(&world, 5).unwrap();
    canvas.export("img/chapter10_yaml.png").unwrap();
}
